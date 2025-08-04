# yt-cli

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![CLI](https://img.shields.io/badge/CLI-Tool-blue)
![YouTube](https://img.shields.io/badge/YouTube-FF0000?logo=youtube&logoColor=white)

Downloads YouTube videos/playlists to your computer. Works with **private and unlisted videos** too if you give it your browser cookies.

## First Time Using Command Line? Read This!

If you've never used a "terminal" or "command line" before, don't worry! Here's what you need to know:

### What is a Terminal/Command Line?

Think of it like a text-based way to talk to your computer. Instead of clicking on things with your mouse, you type commands and press Enter. It sounds scary but it's actually pretty simple.

**What is "bash"?** 
Bash is just the language your computer uses to understand the commands you type. When you see code blocks in this guide that start with ```bash, those are commands you need to type.

### How to Open Terminal:

**On Mac:**
1. Press Cmd+Space (this opens Spotlight search)
2. Type "terminal" 
3. Press Enter
4. A black or white window will open - this is your terminal!

**On Windows:**
1. Press Windows key + R
2. Type "cmd" and press Enter
3. A black window will open - this is your command prompt (same thing as terminal)

**Alternative for Windows:**
1. Click the Start button
2. Type "command prompt" 
3. Click on "Command Prompt" when it appears

### How to Use Commands:

1. **See a command in this guide?** Like `yt-cli --help`
2. **Copy the whole thing** (without the ```bash part)
3. **Paste it in your terminal** (right-click → paste, or Ctrl+V on Windows, Cmd+V on Mac)
4. **Press Enter**
5. **Wait for it to finish** - you'll see text appear, and when it's done, you'll see a new line with a cursor waiting for the next command

**Important:** 
- Don't close the terminal window while commands are running
- If something seems stuck, you can usually stop it by pressing Ctrl+C
- If you see a "%" or "$" at the beginning of a line, that's just showing you where to type - don't type those symbols

### File Paths Explained:

When you see `~/Desktop/cookies.txt`, here's what it means:
- `~` = your home folder (like /Users/YourName on Mac, or C:\Users\YourName on Windows)
- `/` = just separates folders (like clicking through folders)
- So `~/Desktop/cookies.txt` means "the file called cookies.txt on your Desktop"

**Windows users:** Your paths look different - use `C:\Users\YourName\Desktop\cookies.txt` instead

Now let's get started!

## Setup (Step by Step)

### 1. Install yt-dlp first

**On Mac:**
1. Open Terminal app (follow the instructions above if you don't know how)
2. You'll see a window with text and a cursor. This is where you type commands.
3. Copy this command: `brew install yt-dlp`
4. Paste it in the terminal (right-click → paste, or Cmd+V)
5. Press Enter and wait
   - If it says "brew command not found", you need Homebrew first:
   - Copy this entire line: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
   - Paste it in terminal, press Enter, and wait for it to finish (takes 5-10 minutes)
   - Then try `brew install yt-dlp` again

**On Windows:**
1. Open Command Prompt (follow the instructions above)
2. First, make sure you have Python installed:
   - Go to python.org in your web browser
   - Download Python (the big yellow "Download Python" button)
   - Run the installer and make sure to check "Add Python to PATH" during installation
3. Back in Command Prompt, copy this: `pip install yt-dlp`
4. Paste it (right-click → paste) and press Enter
5. Wait for it to download and install

**On Linux:**
1. Open Terminal
2. Copy this: `pip install yt-dlp`
3. Paste and press Enter
4. If pip isn't installed, try: `sudo apt install python3-pip` first (on Ubuntu/Debian)

### 2. Install Rust (needed to compile this tool)

**Everyone needs to do this first:**

1. Go to https://rustup.rs in your web browser
2. You'll see a big command that starts with `curl` - copy the entire line
3. Open Terminal (Mac/Linux) or Command Prompt (Windows)
4. Paste the command you copied and press Enter
5. You'll see text scrolling by - this is normal, just wait
6. When it asks "Proceed with installation (y/n)?", type the letter `y` and press Enter
7. Wait for it to finish (takes 5-10 minutes - you'll see "Rust is installed now" when done)
8. **Important:** Close your terminal completely and open a new one
9. Test that it worked: type `cargo --version` and press Enter
10. If you see a version number like "cargo 1.70.0", you're good! If not, try step 8 again.

### 3. Download and install this tool

**Option A: Download from GitHub (easiest)**
1. Go to this project's GitHub page 
2. Click the green "Code" button
3. Click "Download ZIP"
4. Find the ZIP file in your Downloads folder and double-click to extract it
5. You'll get a folder named something like "yt-cli-main" or "media-saver-main"

**Option B: Use git (if you know what that is)**
```bash
git clone https://github.com/LeonardoCerv/media-saver.git
```

**Now install it:**
1. Open Terminal/Command Prompt (you should know how by now!)
2. Navigate to the folder you just downloaded. Here's how:
   - **The easy way:** Type `cd ` (that's the letters c and d, followed by a space)
   - Then drag the folder from your file explorer (Finder on Mac, File Explorer on Windows) directly into the terminal window
   - Press Enter
   - **The manual way:** Type something like `cd ~/Downloads/yt-cli-main` (but replace the path with where you actually extracted the folder)
3. You should now be "inside" that folder in your terminal
4. Copy this command: `cargo install --path .`
5. Paste it and press Enter
6. **Wait patiently** - this takes 5-15 minutes depending on your computer speed
7. You'll see lots of text scrolling by saying things like "Compiling..." - this is normal!
8. When it's done, you'll see something like "Installed package `yt-cli v0.3.2`"
9. If you see any red error text, something went wrong - check the troubleshooting section

### 4. Test it works

1. **Close your terminal completely** and open a brand new one (this is important!)
2. Type this command: `yt-cli --help`
3. Press Enter
4. If you see a bunch of help text with options listed, congratulations! It's working!
5. If it says "command not found" or similar:
   - Make sure you closed the old terminal and opened a new one
   - Try this instead: `~/.cargo/bin/yt-cli --help`
   - If that works, you need to add ~/.cargo/bin to your PATH (Google "add cargo bin to PATH" + your operating system)

**Final test - actually download something:**
1. Make sure you're in a folder where you want to save videos (like your Desktop)
2. To get to your Desktop in terminal: `cd ~/Desktop` (Mac/Linux) or `cd %USERPROFILE%\Desktop` (Windows)
3. Copy this command: `yt-cli "https://youtube.com/watch?v=dQw4w9WgXcQ"`
4. Paste and press Enter
5. You should see it start downloading! It will create a "downloads" folder and save the video there.
6. If this works, you're all set up!

## Usage

### Basic Examples

**Remember:** These are commands you type in your terminal. Copy each line, paste it in your terminal, and press Enter.

**Download any public video:**
```bash
yt-cli "https://youtube.com/watch?v=dQw4w9WgXcQ"
```
(This downloads to a "downloads" folder in whatever directory your terminal is currently in)

**Download just the audio as an mp3 file:**
```bash
yt-cli -a "https://youtube.com/watch?v=dQw4w9WgXcQ"
```

**Download in a specific quality (like 720p):**
```bash
yt-cli -q 720p "https://youtube.com/watch?v=dQw4w9WgXcQ"
```

**Download an entire playlist:**
```bash
yt-cli -p "https://youtube.com/playlist?list=PLrAXtmRdnEQy5VqrkOEE-1T6GrJhgv79C"
```

**Save to a specific folder (like your Desktop):**
```bash
yt-cli -o ~/Desktop/my-videos "https://youtube.com/watch?v=dQw4w9WgXcQ"
```
(This creates a folder called "my-videos" on your Desktop and saves there)

**Windows users:** For the last example, use this format instead:
```bash
yt-cli -o C:\Users\YourUsername\Desktop\my-videos "https://youtube.com/watch?v=dQw4w9WgXcQ"
```
(Replace "YourUsername" with your actual Windows username)

### For Private/Unlisted Videos

**Step 1: Get your cookies**

1. **Chrome/Edge users:**
   - Open Chrome or Edge
   - Go to YouTube.com and make sure you're logged in to your account
   - Press F12 on your keyboard (or right-click anywhere → "Inspect Element")
   - You'll see developer tools open at the bottom or side of your browser
   - At the top of these tools, look for tabs like "Elements", "Console", "Sources", etc.
   - Click on "Application" (if you don't see it, click the >> arrows to find more tabs)
   - On the left side, you'll see a tree menu - look for "Storage" section
   - Under "Storage", click on "Cookies"
   - Click on "https://www.youtube.com" (there might be other youtube entries, pick the main one)
   - You'll see a big table with cookie data
   - Right-click anywhere in that table area → "Select All" → Right-click again → "Copy"
   - Open Notepad (Windows) or TextEdit (Mac)
   - Paste everything (Ctrl+V or Cmd+V)
   - Save the file as `cookies.txt` on your Desktop
   - Make sure to change the file type to "All Files" when saving (not .txt.txt)

2. **Firefox users:**
   - Open Firefox
   - Go to https://addons.mozilla.org and search for "cookies.txt"
   - Install the "cookies.txt" extension (usually by "Rotemdan")
   - Go to YouTube.com and make sure you're logged in
   - Look for a new cookie icon in your browser toolbar (might be hidden in the extensions menu)
   - Click the cookie icon → "Current Site" → "Export"
   - Save the file as `cookies.txt` on your Desktop

3. **Safari users:**
   - Honestly, just use Chrome or Firefox for this step - Safari makes it unnecessarily complicated
   - If you must use Safari: enable Developer menu first (Safari → Preferences → Advanced → "Show Develop menu"), then it's similar to Chrome but more steps

**Step 2: Use the cookies**

Now when you want to download a private video:

1. Get the URL of the private/unlisted video (copy it from your browser address bar)
2. Open Terminal/Command Prompt
3. Navigate to where you want the video saved (or it will go in a "downloads" folder)
4. Type this command, but replace the parts in CAPS:
```bash
yt-cli -c ~/Desktop/cookies.txt "PUT_THE_VIDEO_URL_HERE"
```

**Real example:**
```bash
yt-cli -c ~/Desktop/cookies.txt "https://youtube.com/watch?v=PRIVATE_VIDEO_ID"
```

**Windows users:** Use this format instead:
```bash
yt-cli -c C:\Users\YourUsername\Desktop\cookies.txt "https://youtube.com/watch?v=PRIVATE_VIDEO_ID"
```
(Replace "YourUsername" with your actual Windows username)

**Important notes:**
- The cookies file needs to be recent - if it doesn't work, export new cookies
- You need to be logged in to the same account that has access to the private video
- This works for unlisted videos, private videos, members-only content, etc.

## All Options

```
yt-cli [OPTIONS] <URL>

OPTIONS:
-o, --output <DIR>    Where to save files (default: ./downloads)
-a, --audio-only      Download audio only (mp3 format)
-p, --playlist        Download entire playlist 
-q, --quality <Q>     Video quality: 144p, 240p, 360p, 480p, 720p, 1080p, 1440p, 2160p
-c, --cookies <FILE>  Cookie file for private videos (see instructions above)
-u, --username <EMAIL> Your YouTube email (alternative to cookies)
--password <PASS>     Your YouTube password (use with username)
--netrc               Use .netrc file for login (advanced users)
-h, --help            Show help
-V, --version         Show version
```

## Troubleshooting

**"yt-dlp not found"**: 
- You skipped step 1. Go back and install yt-dlp first
- Make sure you can type `yt-dlp --version` and see a version number

**"command not found: yt-cli"**: 
- Open a new terminal window (close the old one first)
- If still not working, try the full path: `~/.cargo/bin/yt-cli --help`
- You might need to add Cargo's bin directory to your PATH

**"Invalid URL"**: 
- Make sure you're copying the full YouTube URL from your browser
- It should start with https://youtube.com or https://youtu.be
- Remove any extra characters you might have copied

**Private video fails with cookies**: 
- Make sure your cookies.txt file is recent (export new ones)
- Check that you're logged in to the right YouTube account
- Verify the file path to your cookies.txt is correct
- Make sure the cookies file isn't empty

**"cargo not found"**:
- You skipped installing Rust. Go to https://rustup.rs and install it first
- After installing, close your terminal and open a new one

**Downloads are slow**: 
- That's YouTube limiting download speeds, not this tool
- Try downloading during off-peak hours
- Nothing can be done about this - it's intentional by YouTube

**Permission denied errors**:
- Make sure you have write permission to the output directory
- Try using a different output folder like your Desktop: `yt-cli -o ~/Desktop`

**"Failed to start download"**:
- Make sure yt-dlp is properly installed and working
- Try updating yt-dlp: `pip install --upgrade yt-dlp` or `brew upgrade yt-dlp`

## License

MIT - do whatever you want with it
