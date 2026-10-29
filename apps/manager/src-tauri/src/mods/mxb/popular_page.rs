//! MXB's public top-100 rankings. Parse data, never execute the remote page.
//! Category filtering happens in WordPress before pagination, while the page's
//! rank is restored locally because the posts API ignores ordering parameters.
use super::*;

const MAX_RANKED: usize = 100;
const TTL: Duration = Duration::from_secs(600);
type CacheKey = (String, u32, String);
type RankedCache = HashMap<CacheKey, (Vec<ModSummary>, Instant)>;

fn cache() -> &'static Mutex<RankedCache> {
    static CACHE: std::sync::OnceLock<Mutex<RankedCache>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn page_path(range: &str) -> anyhow::Result<&'static str> {
    match range {
        "all" => Ok("/popular-all-time/"),
        "last30days" => Ok("/popular-last-30-days/"),
        "last7days" => Ok("/popular-last-7-days/"),
        _ => anyhow::bail!("Unsupported popularity period"),
    }
}

fn ranked_slugs(body: &str, base: &str) -> anyhow::Result<Vec<String>> {
    anyhow::ensure!(body.len() <= 2 * 1024 * 1024, "Popularity page is too large");
    let doc = Html::parse_document(body);
    let rows = Selector::parse(".post-content .dmpop-rows .dmpop-title[href]").unwrap();
    let origin = reqwest::Url::parse(base)?;
    let mut slugs = Vec::new();
    for item in doc.select(&rows).take(MAX_RANKED) {
        let Some(href) = item.value().attr("href") else { continue };
        let Ok(url) = origin.join(href) else { continue };
        if url.scheme() != "https" || url.origin() != origin.origin()
            || !url.username().is_empty() || url.password().is_some()
            || url.query().is_some() || url.fragment().is_some() {
            continue;
        }
        let slug = url.path().trim_matches('/');
        if slug.is_empty() || slug.len() > 240
            || !slug.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'%')) {
            continue;
        }
        if !slugs.iter().any(|s| s == slug) { slugs.push(slug.to_string()); }
    }
    anyhow::ensure!(!slugs.is_empty(), "The site's popularity list is unavailable. Choose Newest and try again later.");
    Ok(slugs)
}

fn ranked_posts(posts: &[Value], slugs: &[String], category_id: u32) -> Vec<ModSummary> {
    let mut by_slug: HashMap<String, ModSummary> = posts.iter()
        // Fail closed if the API ignores the requested category filter.
        .filter(|p| p.get("categories").and_then(Value::as_array)
            .is_some_and(|cats| cats.iter().any(|c| c.as_u64() == Some(category_id as u64))))
        .filter_map(|p| summary_from_post(p, category_id))
        .map(|m| (m.slug.clone(), m)).collect();
    slugs.iter().filter_map(|slug| by_slug.remove(slug)).collect()
}

fn page_slice(items: &[ModSummary], page: u32) -> Vec<ModSummary> {
    let start = page.saturating_sub(1) as usize * 24;
    items.iter().skip(start).take(24).cloned().collect()
}

pub(super) async fn search(category_id: u32, page: u32, range: &str) -> anyhow::Result<Vec<ModSummary>> {
    anyhow::ensure!(page > 0, "Page numbers start at 1");
    let path = page_path(range)?;
    if page > 5 { return Ok(Vec::new()); }
    let base = mxb_session::base();
    let key = (base.to_string(), category_id, range.to_string());
    if let Some((items, at)) = lock(cache()).get(&key) {
        if at.elapsed() < TTL { return Ok(page_slice(items, page)); }
    }

    let resp = get_page(&format!("{base}{path}")).await?;
    if !resp.is_success() { return Err(refusal("the public popularity list", &resp)); }
    let slugs = ranked_slugs(&resp.body, base)?;
    let params = vec![
        ("slug", slugs.join(",")),
        ("categories", category_id.to_string()),
        ("per_page", MAX_RANKED.to_string()),
        ("_embed", "author,wp:featuredmedia".to_string()),
    ];
    let resp = get_with_retry(&format!("{base}/wp-json/wp/v2/posts"), &params).await?;
    if !resp.is_success() { return Err(refusal("ranked mod metadata", &resp)); }
    let posts: Vec<Value> = resp.json()?;
    let items = ranked_posts(&posts, &slugs, category_id);
    let result = page_slice(&items, page);
    let mut cached = lock(cache());
    cached.retain(|_, (_, at)| at.elapsed() < TTL);
    if cached.len() >= 64 { cached.clear(); }
    cached.insert(key, (items, Instant::now()));
    Ok(result)
}

#[cfg(test)]
mod local_security_tests {
    use super::*;
    use serde_json::json;
    const BASE: &str = "https://mxb-mods.com";
    fn html(links: &str) -> String {
        format!(r#"<div class="post-content"><ol class="dmpop-rows">{links}</ol></div>"#)
    }
    fn link(url: &str) -> String { format!(r#"<a class="dmpop-title" href="{url}">Track</a>"#) }
    fn post(id: u64, slug: &str, cat: u32) -> Value {
        json!({"id":id,"slug":slug,"title":{"rendered":slug},"categories":[cat]})
    }

    #[test]
    fn current_ranking_markup_retains_site_order_and_deduplicates() {
        let body = html(&(link("/oem-bike-pack/") + &link("/trials-more-pits-version/") + &link("/oem-bike-pack/")));
        assert_eq!(ranked_slugs(&body, BASE).unwrap(), ["oem-bike-pack", "trials-more-pits-version"]);
    }

    #[test]
    fn rankings_reject_executable_external_and_malformed_links() {
        let bad = ["javascript:alert(1)","https://evil.test/a/","https://mxb-mods.com.evil.test/a/",
            "https://user@mxb-mods.com/a/","http://mxb-mods.com/a/","/a/b/","/a/?x=1","/a/#x"];
        let body = html(&(bad.iter().map(|u| link(u)).collect::<String>() + &link("/safe/")));
        assert_eq!(ranked_slugs(&body, BASE).unwrap(), ["safe"]);
    }

    #[test]
    fn missing_rankings_are_an_error_and_never_a_fake_empty_catalog() {
        assert!(ranked_slugs("<p>Temporary maintenance</p>", BASE).is_err());
        assert!(ranked_slugs(&"x".repeat(2 * 1024 * 1024 + 1), BASE).is_err());
    }

    #[test]
    fn rankings_are_bounded_to_the_published_top_100() {
        let body = html(&(0..150).map(|i| link(&format!("/track-{i}/"))).collect::<String>());
        assert_eq!(ranked_slugs(&body, BASE).unwrap().len(), MAX_RANKED);
    }

    #[test]
    fn category_filter_and_site_rank_override_metadata_response_order() {
        let posts = vec![post(3,"third",22),post(2,"bike",29),post(1,"first",22),post(4,"unranked",22)];
        let slugs = vec!["first".into(),"bike".into(),"third".into()];
        let out = ranked_posts(&posts, &slugs, 22);
        assert_eq!(out.iter().map(|m| m.id).collect::<Vec<_>>(), [1,3]);
    }

    #[test]
    fn pagination_ends_after_the_category_subset_without_duplicates() {
        let posts: Vec<_> = (0..30).map(|i| post(i,&format!("track-{i}"),22)).collect();
        let slugs: Vec<_> = (0..30).map(|i| format!("track-{i}")).collect();
        let all = ranked_posts(&posts,&slugs,22);
        assert_eq!(page_slice(&all,1).len(),24);
        assert_eq!(page_slice(&all,2).iter().map(|m| m.id).collect::<Vec<_>>(),(24..30).collect::<Vec<_>>());
        assert!(page_slice(&all,3).is_empty());
    }

    #[test]
    fn popularity_periods_only_select_known_public_pages() {
        assert_eq!(page_path("all").unwrap(),"/popular-all-time/");
        assert_eq!(page_path("last7days").unwrap(),"/popular-last-7-days/");
        assert_eq!(page_path("last30days").unwrap(),"/popular-last-30-days/");
        assert!(page_path("../../anything").is_err());
    }
}
