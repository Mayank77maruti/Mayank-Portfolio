use super::{About, Links, Profile, Repository};
use std::collections::HashMap;

// Ascii art used for Github
const NEOFETCH: &str = include_str!("../../../configs/neofetch.txt");

// Language icons for repos
const RUST: &str = include_str!("../../../configs/lang_icons/rust.txt");
const PYTHON: &str = include_str!("../../../configs/lang_icons/python.txt");
const GITHUB: &str = include_str!("../../../configs/lang_icons/github.txt");

pub fn format_about(about: About) -> String {
    // -------- Experience --------
    let exp_string = about
        .experience
        .iter()
        .map(|exp| {
            format!(
                r#"<span class="blu semibold">Title:</span> {}
<span class="blu semibold">Description:</span>
{}"#,
                exp.title,
                exp.description
                    .iter()
                    .map(|s| format!(r#"<span class="blu semibold">*</span> {}"#, s))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // -------- Education --------
    let edu_string = about
        .education
        .iter()
        .map(|edu| {
            format!(
                r#"<span class="blu semibold">Institute:</span> {}
<span class="blu semibold">Course:</span> {}
<span class="blu semibold">Duration:</span> {}"#,
                edu.institute, edu.course, edu.duration
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // -------- Projects --------
    let projects_string = about.projects.as_ref().map_or(String::new(), |projects| {
        projects
            .iter()
            .map(|p| {
                format!(
                    r#"<span class="blu semibold">Project:</span> {}
<span class="blu semibold">Stack:</span> {}
{}"#,
                    p.name,
                    p.stack.join(", "),
                    p.description
                        .iter()
                        .map(|s| format!(r#"<span class="blu semibold">*</span> {}"#, s))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    });

    // -------- Skills --------
    let skills_string = about.skills.as_ref().map_or(String::new(), |skills| {
        format!(
            r#"<span class="blu semibold">CS:</span> {}
<span class="blu semibold">Frameworks/Tools:</span> {}"#,
            skills.cs_fundamentals.join(", "),
            skills.frameworks_tools.join(", ")
        )
    });

    // -------- Achievements --------
    let achievements_string = about.achievements.as_ref().map_or(String::new(), |ach| {
        ach.iter()
            .map(|a| format!(r#"<span class="blu semibold">*</span> {}"#, a))
            .collect::<Vec<_>>()
            .join("\n")
    });

    // -------- Responsibilities --------
    let responsibilities_string =
        about.responsibilities.as_ref().map_or(String::new(), |res| {
            res.iter()
                .map(|r| format!(r#"<span class="blu semibold">*</span> {}"#, r))
                .collect::<Vec<_>>()
                .join("\n")
        });

    // -------- Final Layout --------
    let final_text = format!(
        r#"<center class="grn semibold">{}</center>
{}

<u class="rd semibold">Languages</u>
{}

<u class="rd semibold">Experience</u>
{}

<u class="rd semibold">Education</u>
{}

<u class="rd semibold">Projects</u>
{}

<u class="rd semibold">Skills</u>
{}

<u class="rd semibold">Achievements</u>
{}

<u class="rd semibold">Responsibilities</u>
{}"#,
        about.name.to_uppercase(),
        about.intro,
        format_langs(about.langs),
        exp_string,
        edu_string,
        projects_string,
        skills_string,
        achievements_string,
        responsibilities_string
    );

    format!(
        r#"<div class="row" style="display:flex; justify-content:center;">
<div class="about">{}</div>
</div>"#,
        final_text
    )
}

pub fn format_profile(profile: Profile) -> String {
    let name = profile.info.name.unwrap_or_else(|| "-".into());
    let bio = profile.info.bio.unwrap_or_else(|| "-".into());
    let company = profile.info.company.unwrap_or_else(|| "-".into());
    let location = profile.info.location.unwrap_or_else(|| "-".into());
    let created_on = &profile.info.created_at[..10];

    let text = format!(
        r#"<a href="https://www.github.com/{username}" target="_blank" style="text-decoration:none">
<span class="grn semibold">{username}</span><span class="grn semibold">@termfolio</span></a>
----------------------
<span class="grn semibold">Name:</span> {name}
<span class="grn semibold">Bio:</span> {bio}
<span class="grn semibold">Repos:</span> {repos}
<span class="grn semibold">Langs:</span> {langs}
<span class="grn semibold">Stars:</span> {stars}
<span class="grn semibold">Forks:</span> {forks}
<span class="grn semibold">Company:</span> {company}
<span class="grn semibold">Location:</span> {location}
<span class="grn semibold">Followers:</span> {followers}
<span class="grn semibold">Following:</span> {following}
<span class="grn semibold">Created on:</span> {created_on}

{BLOCKS}"#,
        username = profile.username,
        name = name,
        bio = bio,
        repos = profile.info.public_repos,
        langs = format_langs(profile.langs),
        stars = profile.stats.stars,
        forks = profile.stats.forks,
        company = company,
        location = location,
        followers = profile.info.followers,
        following = profile.info.following,
        created_on = created_on,
    );

    format!(
        r#"<div class="row">
<div class="ascii">{}</div>
<div class="text">{}</div>
</div>"#,
        NEOFETCH, text
    )
}

pub fn format_repos(repos: &[Repository]) -> String {
    repos
        .iter()
        .map(|repo| {
            let text = format!(
                r#"<a href="https://github.com/{}/{}" target="_blank" class="blu semibold">{}</a>

<span class="rd semibold">Description:</span> {}
<span class="rd semibold">Language:</span> <span class="blu">{}</span>
<span class="rd semibold">Stars:</span> <span class="ylw">{}</span>
<span class="rd semibold">Forks:</span> <span class="ylw">{}</span>"#,
                repo.author,
                repo.name,
                repo.name,
                repo.description,
                repo.language,
                repo.stars,
                repo.forks
            );

            format!(
                r#"<div class="row">
<div class="ascii">{}</div>
<div class="text">{}</div>
</div>"#,
                lang_icon(&repo.language),
                text
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn format_links(links: Links) -> String {
    let mut result = String::new();

    result += &format!(
        r#"<a href="https://github.com/{}" target="_blank" class="semibold" style="color:var(--purple);">
Github</a>: github.com/{}"#,
        links.github, links.github
    );

    if let Some(email) = &links.email {
        result += &format!(
            r#"<br><a href="mailto:{}" class="semibold" style="color:var(--orange);">
Email</a>: {}"#,
            email, email
        );
    }

    if let Some(linkedin) = &links.linkedin {
        result += &format!(
            r#"<br><a href="https://www.linkedin.com/{}" class="semibold" style="color:var(--dblue);">
LinkedIn</a>: linkedin.com/{}"#,
            linkedin, linkedin
        );
    }

    if let Some(twitter) = &links.twitter {
        result += &format!(
            r#"<br><a href="https://www.twitter.com/{}" class="blu semibold">
Twitter/X</a>: @{}"#,
            twitter, twitter
        );
    }

    result
}

pub fn format_langs(langs: Vec<String>) -> String {
    let color_map: HashMap<&str, &str> = [
        ("Rust", "orange"),
        ("Python", "blue"),
        ("C", "dblue"),
        ("C++", "dblue"),
        ("Java", "red"),
        ("Haskell", "purple"),
        ("Zig", "orange"),
        ("Go", "blue"),
        ("Dart", "dblue"),
        ("JavaScript", "yellow"),
        ("TypeScript", "blue"),
        ("Bash", "dgreen"),
    ]
    .into();

    langs
        .into_iter()
        .map(|lang| {
            color_map.get(lang.as_str()).map_or_else(
                || format!(r#"<span>{}</span>"#, lang),
                |color| format!(r#"<span style="color:var(--{});">{}</span>"#, color, lang),
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn lang_icon(lang: &str) -> &str {
    match lang {
        "Rust" => RUST,
        "Python" | "Jupyter Notebook" => PYTHON,
        _ => GITHUB,
    }
}

const BLOCKS: &str = r#"<span class="blocks" style="color:var(--black)">█</span><span class="rd blocks">█</span><span class="grn blocks">█</span><span class="ylw blocks">█</span><span class="blu blocks">█</span><span class="blocks" style="color:var(--orange)">█</span><span class="blocks" style="color:var(--purple)">█</span><span class="blocks">█</span>"#;
