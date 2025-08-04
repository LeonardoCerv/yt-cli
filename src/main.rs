use anyhow::{Context, Result};
use clap::{Arg, Command};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::process::{Command as StdCommand, Stdio};
use std::thread;
use url::Url;

const APP_NAME: &str = "yt-cli";
const VERSION: &str = "0.3.2";

#[tokio::main]
async fn main() -> Result<()> {
    let app = setup_cli();
    let matches = app.get_matches();

    // check if yt-dlp is installed first
    if !verify_dependencies().await? {
        return Ok(());
    }

    let opts = parse_args(&matches);
    
    check_url(&opts.url)?;
    create_output_dir(&opts.output_path)?;

    show_info(&opts);
    
    download_video(&opts).await?;
    
    println!("{}", "✨ All done!".bright_green().bold());
    Ok(())
}

#[derive(Debug)]
struct DownloadOpts {
    url: String,
    output_path: String,
    format: String,
    audio_only: bool,
    playlist: bool,
    quality: Option<String>,
    cookies: Option<String>,
    username: Option<String>,
    password: Option<String>,
    use_netrc: bool,
}

fn setup_cli() -> Command {
    Command::new(APP_NAME)
        .version(VERSION)
        .about("Download YouTube videos/playlists quickly")
        .arg(
            Arg::new("url")
                .help("YouTube URL to download")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Output directory")
                .default_value("./downloads")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Format preference")
                .default_value("best")
        )
        .arg(
            Arg::new("audio-only")
                .short('a')
                .long("audio-only")
                .action(clap::ArgAction::SetTrue)
                .help("Download audio only (mp3)")
        )
        .arg(
            Arg::new("playlist")
                .short('p')
                .long("playlist")
                .action(clap::ArgAction::SetTrue)
                .help("Download whole playlist")
        )
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .value_name("QUALITY")
                .help("Video quality (e.g., 720p, 1080p)")
        )
        .arg(
            Arg::new("cookies")
                .short('c')
                .long("cookies")
                .value_name("FILE")
                .help("Cookie file for private videos")
        )
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .value_name("EMAIL")
                .help("Account email")
        )
        .arg(
            Arg::new("password")
                .long("password")
                .value_name("PASS")
                .help("Account password")
        )
        .arg(
            Arg::new("netrc")
                .long("netrc")
                .action(clap::ArgAction::SetTrue)
                .help("Use .netrc file for auth")
        )
}

fn parse_args(matches: &clap::ArgMatches) -> DownloadOpts {
    DownloadOpts {
        url: matches.get_one::<String>("url").unwrap().clone(),
        output_path: matches.get_one::<String>("output").unwrap().clone(),
        format: matches.get_one::<String>("format").unwrap().clone(),
        audio_only: matches.get_flag("audio-only"),
        playlist: matches.get_flag("playlist"),
        quality: matches.get_one::<String>("quality").cloned(),
        cookies: matches.get_one::<String>("cookies").cloned(),
        username: matches.get_one::<String>("username").cloned(),
        password: matches.get_one::<String>("password").cloned(),
        use_netrc: matches.get_flag("netrc"),
    }
}

async fn verify_dependencies() -> Result<bool> {
    print!("{}", "Checking for yt-dlp... ".dimmed());
    
    match StdCommand::new("yt-dlp").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("{}", format!("found v{}", ver).green());
            Ok(true)
        }
        _ => {
            println!("{}", "not found".red());
            println!("\n{}", "You need to install yt-dlp first:".yellow().bold());
            println!("  macOS: brew install yt-dlp");
            println!("  pip: pip install yt-dlp");
            println!("  or get it from: https://github.com/yt-dlp/yt-dlp\n");
            Ok(false)
        }
    }
}

fn check_url(url: &str) -> Result<()> {
    let parsed = Url::parse(url).context("Invalid URL")?;
    let host = parsed.host_str().context("No host found")?;

    let yt_domains = ["youtube.com", "youtu.be", "music.youtube.com"];
    
    if !yt_domains.iter().any(|&domain| host.contains(domain)) {
        anyhow::bail!("Not a YouTube URL");
    }
    
    Ok(())
}

fn create_output_dir(path: &str) -> Result<()> {
    std::fs::create_dir_all(path)
        .with_context(|| format!("Can't create directory: {}", path))
}

fn show_info(opts: &DownloadOpts) {
    println!("\n{}", "yt-cli".bright_blue().bold());
    println!("{}", "─".repeat(40).bright_blue());
    println!("🎬 URL: {}", opts.url);
    println!("� Save to: {}", opts.output_path);
    
    if opts.audio_only {
        println!("🎵 Audio only");
    } else if let Some(ref quality) = opts.quality {
        println!("📺 Quality: {}", quality);
    }
    
    if opts.playlist {
        println!("📋 Whole playlist");
    }
    
    // auth stuff
    if opts.cookies.is_some() {
        println!("🍪 Using cookies");
    } else if opts.username.is_some() {
        println!("� Using login");
    } else if opts.use_netrc {
        println!("� Using .netrc");
    }
    
    println!();
}

async fn download_video(opts: &DownloadOpts) -> Result<()> {
    let mut cmd = StdCommand::new("yt-dlp");
    
    // basic stuff
    cmd.arg(&opts.url)
       .arg("-o").arg(format!("{}/%(title)s.%(ext)s", opts.output_path))
       .arg("--no-warnings");

    // auth
    setup_auth(&mut cmd, opts);
    
    // format
    if opts.audio_only {
        cmd.arg("-x").arg("--audio-format").arg("mp3").arg("--audio-quality").arg("0");
    } else {
        setup_video_format(&mut cmd, opts);
    }

    // playlist stuff
    if opts.playlist {
        cmd.arg("--yes-playlist");
    } else {
        cmd.arg("--no-playlist");
    }

    // do the download
    run_download(cmd).await
}

fn setup_auth(cmd: &mut StdCommand, opts: &DownloadOpts) {
    if let Some(ref cookies_file) = opts.cookies {
        cmd.arg("--cookies").arg(cookies_file);
    }

    if let Some(ref username) = opts.username {
        cmd.arg("--username").arg(username);
        if let Some(ref password) = opts.password {
            cmd.arg("--password").arg(password);
        }
    }

    if opts.use_netrc {
        cmd.arg("--netrc");
    }
}

fn setup_video_format(cmd: &mut StdCommand, opts: &DownloadOpts) {
    if let Some(ref quality) = opts.quality {
        let format_str = match quality.as_str() {
            "144p" => "worst[height<=144]",
            "240p" => "best[height<=240]", 
            "360p" => "best[height<=360]",
            "480p" => "best[height<=480]",
            "720p" => "best[height<=720]",
            "1080p" => "best[height<=1080]",
            "1440p" => "best[height<=1440]",
            "2160p" => "best[height<=2160]",
            _ => &opts.format,
        };
        cmd.arg("-f").arg(format_str);
    } else {
        cmd.arg("-f").arg(&opts.format);
    }
}

async fn run_download(mut cmd: StdCommand) -> Result<()> {
    cmd.arg("--newline").arg("--progress")
       .stdout(Stdio::piped()).stderr(Stdio::piped());

    println!("Starting download...\n");

    let mut child = cmd.spawn().context("Failed to start download")?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );

    let mut error_lines = Vec::new();
    
    // handle stdout
    let stdout_handle = child.stdout.take().map(|stdout| {
        let spinner_clone = spinner.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let messages = Vec::new();
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("[download]") {
                        spinner_clone.set_message(line.trim().to_string());
                        spinner_clone.tick();
                    } else if !line.trim().is_empty() {
                        print!("\r{}", line.trim());
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                    }
                }
            }
            messages
        })
    });

    // handle stderr
    let stderr_handle = child.stderr.take().map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut messages = Vec::new();
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    if !line.trim().is_empty() && !line.contains("WARNING") {
                        messages.push(line.trim().to_string());
                    }
                }
            }
            messages
        })
    });

    // wait for threads
    if let Some(handle) = stdout_handle {
        if let Ok(stream_errors) = handle.join() {
            error_lines.extend(stream_errors);
        }
    }

    if let Some(handle) = stderr_handle {
        if let Ok(stream_errors) = handle.join() {
            error_lines.extend(stream_errors);
        }
    }

    spinner.finish_with_message("Done".to_string());

    let status = child.wait().context("Download failed")?;

    if !status.success() {
        if !error_lines.is_empty() {
            eprintln!("\n{}", "Something went wrong:".red().bold());
            for error in error_lines.iter().take(3) {
                eprintln!("  {}", error.trim());
            }
        }
        anyhow::bail!("Download failed");
    }

    Ok(())
}
