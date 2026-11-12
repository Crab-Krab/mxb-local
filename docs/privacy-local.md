# MXB Local: what the app contacts

Applies to builds after 0.20.6-local.3. In local.3 itself, the startup request, the server-list
upload and the supporters fetch listed under "Never" below still happen.

This note is specific to MXB Local. [`privacy.md`](privacy.md) is the upstream document for
MXB App, MXB Studio, MXB Coach and FrostMod, and describes data collection that this fork has
turned off.

MXB Local sends no usage statistics, crash reports, game-memory reports or machine
identifiers. It does not check for updates. The list below is everything else it reaches, and
when.

## Only when you use a feature

| When | Who is contacted | What they learn |
| --- | --- | --- |
| Browsing mods | mxb-mods.com, gpb-mods.com, mxbikes-shop.com (and its CDN), mxb-hub.com | Your IP address and the pages you open, as with any website. |
| Viewing a mod description | The same catalog hosts | Images on any other host are removed rather than loaded. |
| Installing a mod | The download host the mod uses (for example MediaFire, MEGA, Google Drive) | Your IP address and the file you download. |
| Installing an MX Bikes mod from mxb-mods.com | api.mxbsecure.com (the upstream developer's service) | Which mod you are installing. The app asks whether the service holds a copy of the file and uses that copy if it does. If it doesn't, the service is asked to make one. |
| Opening the Online (servers) screen | api.mxbsecure.com | Your IP address. The app reads a shared server list from there when your own list is empty or the game's master server fails to answer. Nothing is uploaded. |
| Signing in to the shop or MXB Hub | That store | What any sign-in to that site shares. The login is stored encrypted for your Windows user. |
| Linking an upstream account, friends, uploads | api.mxbsecure.com | What that feature needs. Friends presence is off unless you turn it on. |

## Never, in this fork

- A request at startup. Upstream read the shared server list on every fresh install.
- Uploading the game server list you see. Upstream sent it to api.mxbsecure.com while the
  Online screen was open.
- Fetching the supporters list from GitHub when Settings opens.
- Downloading and running programs (FrostMod, the Visual C++ runtime installer).

Requests the app makes itself identify as `mxb-local`. Your IP address is visible to every
service above, as with any internet connection.
