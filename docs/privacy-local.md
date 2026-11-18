# MXB Local: what the app contacts

Applies to 1.1.0 and later. In local.3 and earlier, the startup request, the server-list upload
and the supporters fetch listed under "Never" below still happen. Up to 1.0.0, the Online
section, the upstream account and share codes still work as described in "Before 1.1.0" below.

**Correction for local.4.** local.4 still asks GitHub (`api.github.com`) for the latest
FrostMod release from the upstream developer's repositories, at launch and every 30 minutes.
GitHub sees your IP address and that the request came from `mxb-local`. Nothing else is sent,
and nothing is downloaded. An earlier version of this note missed it. local.5 and later do not
make this request.

This note is specific to MXB Local. [`privacy.md`](privacy.md) is the upstream document for
MXB App, MXB Studio, MXB Coach and FrostMod, and describes data collection that this fork has
turned off.

MXB Local sends no usage statistics, crash reports, game-memory reports or machine
identifiers. It does not check for updates to itself or to FrostMod. The list below is everything else it reaches, and
when.

## Only when you use a feature

| When | Who is contacted | What they learn |
| --- | --- | --- |
| Browsing mods | mxb-mods.com, gpb-mods.com, mxbikes-shop.com (and its CDN), mxb-hub.com | Your IP address and the pages you open, as with any website. |
| Viewing a mod description | The same catalog hosts | Images on any other host are removed rather than loaded. |
| Installing a mod | The download host the mod uses (for example MediaFire, MEGA, Google Drive) | Your IP address and the file you download. |
| Installing an MX Bikes mod from mxb-mods.com | api.mxbsecure.com (the upstream developer's service) | Which mod you are installing. The app asks whether the service holds a copy of the file and uses that copy if it does. If it doesn't, the service is asked to make one. |
| Signing in to the shop or MXB Hub | That store | What any sign-in to that site shares. The login is stored encrypted for your Windows user. |

## Never, in this fork

- A request at startup. Upstream read the shared server list on every fresh install.
- Uploading the game server list you see. Upstream sent it to api.mxbsecure.com while the
  Online screen was open.
- Fetching the supporters list from GitHub when Settings opens.
- Asking GitHub for FrostMod releases (see the correction above for local.4).
- Downloading and running programs (FrostMod, the Visual C++ runtime installer).
- Anything from the Online section: signing in to the MX Bikes master server with your Steam
  account, pinging game servers, the shared server list, friends and where you are playing,
  the queue for a full server, and Ranked (mxb-ranked.com).
- An account on the original developer's service. A saved one is deleted from your PC when
  1.1.0 or later first starts. Nothing is sent to close it on their side.
- Uploading to, or downloading from, a file host for share codes or "Share logs".

## Before 1.1.0

**Correction.** Earlier versions of this note did not list share codes. Up to 1.0.0, sharing
installed content or a preset, and "Share logs" in Settings, uploaded the files to
files.catbox.moe, a public file host, and installing someone's share code downloaded from it.
This only happened when you used those buttons.

Up to 1.0.0, the Online screen also:

- signed in to the MX Bikes master server with your Steam account and pinged every listed
  server, refreshing every couple of minutes while the screen was open
- read a shared server list from api.mxbsecure.com when your own list was empty or the master
  server didn't answer
- created an account on api.mxbsecure.com the first time a feature needed one, such as Friends
  or the queue. With
  "Show friends which server I'm on" switched on, it reported the server you were on. While
  the game ran, it also sent your game ID and rider name to that account.
- asked mxb-ranked.com for your rank when you opened Ranked

Requests the app makes itself identify as `mxb-local`. Your IP address is visible to every
service above, as with any internet connection.
