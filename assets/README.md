# Cargo-Forge Assets

This directory contains visual assets for the Cargo-Forge project documentation.

## Required Assets

### cargo-forge-demo.gif

An animated GIF demonstrating the cargo-forge tool in action. The GIF should show:

1. **Initial Command** (0-3s)
   - Terminal showing: `cargo forge new`
   - Clear, readable font (recommended: 14-16px)

2. **Interactive Prompts** (3-15s)
   - Project name entry: "my-awesome-api"
   - Project type selection menu with arrow navigation
   - Selecting "API Server"
   - Feature selection:
     - Authentication: JWT (selected)
     - Database: PostgreSQL (selected)
     - Docker support: Yes
     - CI/CD: GitHub Actions

3. **Generation Progress** (15-18s)
   - Progress indicators showing:
     - Creating project structure...
     - Generating templates...
     - Setting up features...
     - Initializing git repository...

4. **Success Message** (18-20s)
   - Colorful success message
   - Project location
   - Next steps hint

### Technical Requirements

- **Dimensions**: 800x600px (or similar 4:3 ratio)
- **Frame rate**: 10-15 fps
- **File size**: < 5MB
- **Colors**: High contrast, dark terminal theme
- **Speed**: Normal typing speed, with slight pauses at decision points

### Tools for Creating GIF

Recommended tools:
- **asciinema** + **agg**: Record terminal and convert to GIF
- **terminalizer**: Terminal recorder with built-in GIF export
- **ttygif**: Simple terminal to GIF converter
- **vhs**: Declarative terminal GIF generator

### Example VHS Script

```tape
# cargo-forge-demo.tape
# Use with vhs: https://github.com/charmbracelet/vhs

Output cargo-forge-demo.gif

Set FontSize 16
Set Width 800
Set Height 600
Set Theme "Dracula"

Type "cargo forge new"
Enter
Sleep 2s

Type "my-awesome-api"
Enter
Sleep 1s

Down
Down
Enter
Sleep 1s

Down
Enter
Sleep 500ms

Down
Enter
Sleep 500ms

Enter
Sleep 500ms

Down
Enter
Sleep 3s

Sleep 2s
```

### Additional Assets (Optional)

- `logo.png`: High-resolution Cargo-Forge logo
- `architecture.png`: Diagram showing project structure
- `comparison-chart.png`: Visual comparison with cargo-generate
- `screenshots/`: Directory with UI screenshots

## Contributing Assets

When adding new assets:
1. Optimize file sizes (use tools like gifsicle for GIFs)
2. Ensure assets are accessible (good contrast, readable text)
3. Include source files if applicable
4. Update this README with asset descriptions