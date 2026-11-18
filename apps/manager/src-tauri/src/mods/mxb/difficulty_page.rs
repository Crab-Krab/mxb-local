//! The Beginner, Intermediate and Pro track filters, read from mxb-mods.com's public
//! `/track-difficulty/{level}/` pages.
//!
//! These used to be WordPress categories 300, 301 and 302. In November 2026 the site moved
//! them to a separate "track difficulty" list that its REST API does not expose, so the
//! category ids answer "Term does not exist". The app keeps the old ids as the filters'
//! names, reads each difficulty page for which tracks are on it and in what order, then
//! asks the REST API for those posts, as the Popular reader does. Data only: nothing from
//! the page is run.
use super::popular_page::{post_slug, ranked_posts};
use super::*;

/// The Tracks category. Every post on a difficulty page is in it, and a post that isn't is
/// left out rather than shown under the wrong type.
const TRACKS: u32 = 22;
/// Cards on one of the site's difficulty pages.
const SITE_PAGE: usize = 16;
/// Results on one of the app's pages (`PER_PAGE`).
const APP_PAGE: usize = 24;
/// The app pages the site is asked for, at most (2,400 tracks per difficulty). A backstop
/// against a site that never answers "no such page".
const MAX_APP_PAGE: u32 = 100;
const TTL: Duration = Duration::from_secs(600);

/// The difficulty a filter id stands for, or `None` for an ordinary category.
pub(super) fn level(category_id: u32) -> Option<&'static str> {
    match category_id {
        300 => Some("beginner"),
        301 => Some("intermediate"),
        302 => Some("pro"),
        _ => None,
    }
}

/// One site page's slugs, or `None` once the site says the page does not exist.
type SitePage = Option<Vec<String>>;
type Cache = HashMap<(String, &'static str, u32), (SitePage, Instant)>;

fn cache() -> &'static Mutex<Cache> {
    static CACHE: std::sync::OnceLock<Mutex<Cache>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn page_url(base: &str, level: &str, site_page: u32) -> String {
    if site_page == 1 {
        format!("{base}/track-difficulty/{level}/")
    } else {
        format!("{base}/track-difficulty/{level}/page/{site_page}/")
    }
}

/// The track slugs on one difficulty page, in the site's order. Each card is a link with
/// the post's id (`<a href="…/slug/" id="post-123" class="post type-post …">`).
///
/// A page that loads but holds no cards is an error, not an empty list: that is what a
/// change to the site's layout looks like, and an empty grid would claim there are no
/// tracks of that difficulty.
fn card_slugs(body: &str, base: &str) -> anyhow::Result<Vec<String>> {
    anyhow::ensure!(body.len() <= 2 * 1024 * 1024, "Difficulty page is too large");
    let doc = Html::parse_document(body);
    let cards = Selector::parse(r#"a.type-post[href][id^="post-"]"#).unwrap();
    let origin = reqwest::Url::parse(base)?;
    let mut slugs = Vec::new();
    for card in doc.select(&cards).take(SITE_PAGE * 2) {
        let Some(slug) = card.value().attr("href").and_then(|h| post_slug(&origin, h)) else {
            continue;
        };
        if !slugs.contains(&slug) {
            slugs.push(slug);
        }
    }
    anyhow::ensure!(
        !slugs.is_empty(),
        "The site's track difficulty list is unavailable. Choose All and try again later."
    );
    Ok(slugs)
}

/// Which site pages hold the app's page `page`, and where in the first of them it starts.
fn site_pages_for(page: u32) -> (std::ops::RangeInclusive<u32>, usize) {
    let start = (page as usize - 1) * APP_PAGE;
    let end = start + APP_PAGE;
    let first = start / SITE_PAGE + 1;
    let last = (end - 1) / SITE_PAGE + 1;
    (first as u32..=last as u32, start - (first - 1) * SITE_PAGE)
}

/// The app's page `page` from the site pages that hold it, given in order. `None` marks a
/// page past the end, after which nothing more is read.
fn app_slice(site_pages: &[SitePage], offset: usize) -> Vec<String> {
    site_pages
        .iter()
        .map_while(|p| p.as_ref())
        .flatten()
        .skip(offset)
        .take(APP_PAGE)
        .cloned()
        .collect()
}

async fn site_page(base: &str, level: &'static str, n: u32) -> anyhow::Result<SitePage> {
    let key = (base.to_string(), level, n);
    if let Some((p, at)) = lock(cache()).get(&key) {
        if at.elapsed() < TTL {
            return Ok(p.clone());
        }
    }
    let resp = get_page(&page_url(base, level, n)).await?;
    let page = if resp.status == 404 {
        None
    } else if resp.is_success() {
        Some(card_slugs(&resp.body, base)?)
    } else {
        return Err(refusal("the track difficulty list", &resp));
    };
    let mut cached = lock(cache());
    cached.retain(|_, (_, at)| at.elapsed() < TTL);
    if cached.len() >= 256 {
        cached.clear();
    }
    cached.insert(key, (page.clone(), Instant::now()));
    Ok(page)
}

/// One page of tracks of the given difficulty, in the site's order.
pub(super) async fn search(level: &'static str, page: u32) -> anyhow::Result<Vec<ModSummary>> {
    anyhow::ensure!(page > 0, "Page numbers start at 1");
    if page > MAX_APP_PAGE {
        return Ok(Vec::new());
    }
    let base = mxb_session::base();
    anyhow::ensure!(
        base == mxb_session::MXB_SITE.base,
        "Track difficulty is only available on mxb-mods.com"
    );

    let (pages, offset) = site_pages_for(page);
    let mut site_pages = Vec::new();
    for n in pages {
        let p = site_page(base, level, n).await?;
        let end = p.is_none();
        site_pages.push(p);
        if end {
            break;
        }
    }
    let slugs = app_slice(&site_pages, offset);
    if slugs.is_empty() {
        return Ok(Vec::new());
    }

    let params = vec![
        ("slug", slugs.join(",")),
        ("categories", TRACKS.to_string()),
        ("per_page", "100".to_string()),
        ("_embed", "author,wp:featuredmedia".to_string()),
    ];
    let resp = get_with_retry(&format!("{base}/wp-json/wp/v2/posts"), &params).await?;
    if !resp.is_success() {
        return Err(refusal("track metadata", &resp));
    }
    let posts: Vec<Value> = resp.json()?;
    Ok(ranked_posts(&posts, &slugs, TRACKS))
}

#[cfg(test)]
mod tests {
    use super::*;
    const BASE: &str = "https://mxb-mods.com";

    fn card(slug: &str, id: u32) -> String {
        format!(
            r#"<a href="https://mxb-mods.com/{slug}/" id="post-{id}" class="post type-post status-publish hentry "><h2 class="archive-post-title">{slug}</h2></a>"#
        )
    }

    fn slugs(range: std::ops::Range<u32>) -> SitePage {
        Some(range.map(|i| format!("t{i}")).collect())
    }

    #[test]
    fn only_the_three_old_difficulty_ids_are_difficulties() {
        assert_eq!(level(300), Some("beginner"));
        assert_eq!(level(301), Some("intermediate"));
        assert_eq!(level(302), Some("pro"));
        assert_eq!(level(22), None);
        assert_eq!(level(119), None);
    }

    #[test]
    fn the_first_page_has_no_page_number_in_its_address() {
        assert_eq!(page_url(BASE, "pro", 1), "https://mxb-mods.com/track-difficulty/pro/");
        assert_eq!(
            page_url(BASE, "pro", 3),
            "https://mxb-mods.com/track-difficulty/pro/page/3/"
        );
    }

    #[test]
    fn cards_are_read_in_order_and_unsafe_links_are_skipped() {
        let body = format!(
            "<main>{}{}{}{}</main><aside><a href=\"/sidebar/\">Not a card</a></aside>",
            card("snow-moto-2", 198267),
            r#"<a href="https://evil.test/x/" id="post-1" class="post type-post">x</a>"#,
            card("bar2bar-2", 195579),
            card("snow-moto-2", 198267),
        );
        assert_eq!(card_slugs(&body, BASE).unwrap(), ["snow-moto-2", "bar2bar-2"]);
    }

    #[test]
    fn a_page_with_no_cards_is_an_error_not_an_empty_list() {
        assert!(card_slugs("<p>Maintenance</p>", BASE).is_err());
    }

    #[test]
    fn app_pages_of_24_are_built_from_site_pages_of_16() {
        // App page 1 is site pages 1 and 2 (the first 8 of page 2).
        assert_eq!(site_pages_for(1), (1..=2, 0));
        // App page 2 starts halfway through site page 2 and ends with site page 3.
        assert_eq!(site_pages_for(2), (2..=3, 8));
        assert_eq!(site_pages_for(3), (4..=5, 0));

        let page2 = app_slice(&[slugs(16..32), slugs(32..48)], 8);
        assert_eq!(page2.first().map(String::as_str), Some("t24"));
        assert_eq!(page2.last().map(String::as_str), Some("t47"));
        assert_eq!(page2.len(), 24);
    }

    #[test]
    fn the_list_ends_at_the_first_page_the_site_does_not_have() {
        // 20 tracks in all: app page 1 is short, which is how the grid knows it is the end.
        let page1 = app_slice(&[slugs(0..16), slugs(16..20)], 0);
        assert_eq!(page1.len(), 20);
        let past = app_slice(&[None, slugs(0..16)], 0);
        assert!(past.is_empty());
    }
}
