pub const HELP: &str = r#"<span class="grn"> _____________  __  ___________  __   ________ __________
.___  ___.      ___   ____    ____  ___      .__   __.  __  ___ 
|   \/   |     /   \  \   \  /   / /   \     |  \ |  | |  |/  / 
|  \  /  |    /  ^  \  \   \/   / /  ^  \    |   \|  | |  '  /  
|  |\/|  |   /  /_\  \  \_    _/ /  /_\  \   |  . `  | |    <   
|  |  |  |  /  _____  \   |  |  /  _____  \  |  |\   | |  .  \  
|__|  |__| /__/     \__\  |__| /__/     \__\ |__| \__| |__|\__\  

            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⣤⣤⣤⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⢀⣠⣶⣿⣿⡿⠿⠿⠿⠿⢿⣿⣿⣷⣦⣄⣀⣤⣶⣶
            ⠀⠀⠀⠀⠀⣰⣿⣿⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠉⠛⠿⣿⣿⣿⠟⠋
            ⠀⠀⠀⠀⣼⣿⡿⠃⠀⢀⣤⣾⣿⣿⣿⣿⣷⣦⣄⠀⠀⠈⠉⠀⠀⠀
            ⠀⠀⠀⣸⣿⡿⠁⠀⢠⣿⣿⠟⠉⠀⠈⠉⠛⢿⣿⣷⡄⠀⠀⠀⠀⠀
            ⠀⠀⢀⣿⣿⡇⠀⠀⣾⣿⡟⠀⠀⢀⣤⣄⠀⠀⠹⣿⣿⡄⠀⠀⠀⠀
            ⠀⠀⣾⣿⣿⡇⠀⠀⢻⣿⣷⡀⠀⠘⣿⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀
            ⠀⣼⣿⡿⣿⣿⡄⠀⠈⠻⣿⣿⣷⣿⣿⡿⠃⠀⢀⣿⣿⡇⠀⠀⠀⠀
            ⣰⣿⣿⠁⠹⣿⣿⣦⡀⠀⠈⠉⠛⠋⠉⠀⠀⣠⣾⣿⡟⠀⠀⠀⠀⠀
            ⣿⣿⣧⣤⣤⣬⣿⣿⣿⣶⣦⣤⣤⣤⣴⣶⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀
            ⠙⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀
</span>

Hello World
Type <i>help</i> to see the list of available commands
"#;

pub const CREDITS: &str = r#"<span class="grn"> _____________  __  ___________  __   ________
/_  __/ __/ _ \/  |/  / __/ __ \/ /  /  _/ __ \
 / / / _// , _/ /|_/ / _// /_/ / /___/ // /_/ /
/_/ /___/_/|_/_/  /_/_/  \____/____/___/\____/ 
</span>
Terminal style portfolio website. 
 
  <a href="https://github.com/Mayank77maruti" target="_blank" class="blu semibold">Github</a>: github.com/Mayank77maruti

  <a href="https://github.com/Mayank77maruti/termfolio" target="_blank" class="blu semibold">Repo</a>: github.com/Mayank77maruti/termfolio

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
