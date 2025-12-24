pub const HELP: &str = r#"<span class="grn">
</span>

<table>
  <tr>
    <td><span class="rd semibold">about</span></td>
    <td>View about me</td>
  </tr>
  <tr>
    <td><span class="rd semibold">neofetch / fastfetch / github</span></td>
    <td>View GitHub profile</td>
  </tr>
  <tr>
    <td><span class="rd semibold">onefetch / repos</span></td>
    <td>View pinned repos / projects</td>
  </tr>
  <tr>
    <td><span class="rd semibold">links</span></td>
    <td>View contact info and links</td>
  </tr>
  <tr>
    <td><span class="rd semibold">help</span></td>
    <td>View this help section</td>
  </tr>
  <tr>
    <td><span class="rd semibold">theme / wal</span></td>
    <td>Cycle through themes</td>
  </tr>
  <tr>
    <td><span class="rd semibold">credits</span></td>
    <td>View credits and repository</td>
  </tr>
  <tr>
    <td><span class="rd semibold">history</span></td>
    <td>View command history</td>
  </tr>
  <tr>
    <td><span class="rd semibold">clear</span></td>
    <td>Clear screen</td>
  </tr>
</table>

<br />

You can use <i>arrow keys</i> to scroll through history,<br />
and <i>Ctrl + L</i> to clear the screen.<br /><br />
"#;

pub const CREDITS: &str = r#"<span class="grn"> _____________  __  ___________  __   ________
/_  __/ __/ _ \/  |/  / __/ __ \/ /  /  _/ __ \
 / / / _// , _/ /|_/ / _// /_/ / /___/ // /_/ /
/_/ /___/_/|_/_/  /_/_/  \____/____/___/\____/ 
</span>
Terminal style portfolio website. 
 
  <a href="https://github.com/shettysach" target="_blank" class="blu semibold">Github</a>: github.com/shettysach

  <a href="https://github.com/shettysach/termfolio" target="_blank" class="blu semibold">Repo</a>: github.com/shettysach/termfolio

<span class="rd semibold">APIs used -</span>

* <a 
    href="https://docs.github.com/en/rest/about-the-rest-api"
    target="_blank"
    class="blu semibold">Github REST API</a>

* <a 
    href="https://pinned.berrysauce.me"
    target="_blank" 
    class="blu semibold">Pinned repos</a> - berrysauce/pinned

* <a 
    href="https://github.com/idealclover/GitHub-Star-Counter"
    target="_blank"
    class="blu semibold">Total stars and forks</a> - idealclover/GitHub-Star-Counter

"#;

pub const READ_JSON_ERROR: &str = r#"<span class="rd semibold">Error reading config.json</span>"#;
pub const FETCH_GITHUB_ERROR: &str =
    r#"<span class="rd semibold">Error fetching data from Github.</span>"#;