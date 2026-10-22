use termcolor::Color;
use termcolor::ColorSpec;
use termcolor::WriteColor;
const NAME: &str = env!("CARGO_PKG_NAME");
/// (flags, arguments, description) - one row per line in the COMMANDS section.
const COMMANDS: &[(&str, &str, &str)] = &[
    ("-l, --list", "", "Show all tasks and their sub-tasks"),
    ("-a, --add", "<text>", "Add a new task"),
    (
        "-a, --add",
        "<id|name> <text>",
        "Add a sub-task to an existing task",
    ),
    (
        "-d, --delete",
        "<id>",
        "Delete a task together with its sub-tasks",
    ),
    ("-d, --delete", "<id> <sub-id>", "Delete a single sub-task"),
    (
        "-c, --complete",
        "<id> [sub-id]",
        "Mark a task (or sub-task) as done",
    ),
    (
        "-i, --incomplete",
        "<id> [sub-id]",
        "Mark a task (or sub-task) as not done",
    ),
    ("-h, --help", "", "Show this help"),
];

const EXAMPLES: &[&str] = &[
    "--add \"Buy milk\"",
    "--add 1 \"Oat milk, 1 l\"",
    "-c 1 2",
    "--list",
];

fn paint(
    out: &mut impl WriteColor,
    color: Option<Color>,
    bold: bool,
    text: &str,
) -> std::io::Result<()> {
    out.set_color(ColorSpec::new().set_fg(color).set_bold(bold))?;
    write!(out, "{text}")?;
    out.reset()
}

fn heading(out: &mut impl WriteColor, text: &str) -> std::io::Result<()> {
    paint(out, Some(Color::Yellow), true, text)?;
    writeln!(out)
}

/// Prints usage information for `-h` / `--help`.
pub fn print_help(out: &mut impl WriteColor) -> std::io::Result<()> {
    paint(out, None, true, NAME)?;
    writeln!(
        out,
        " {} - simple to-do list with sub-tasks\n",
        env!("CARGO_PKG_VERSION")
    )?;

    heading(out, "USAGE:")?;
    write!(out, "    ")?;
    paint(out, None, true, NAME)?;
    write!(out, " ")?;
    paint(out, Some(Color::Magenta), false, "<COMMAND> [ARGS]")?;
    writeln!(out, "\n")?;

    heading(out, "COMMANDS:")?;
    // Coloring splits a line into pieces, so `{:<width$}` can't align it - pad by hand.
    let visible =
        |flags: &str, args: &str| flags.len() + if args.is_empty() { 0 } else { 1 + args.len() };
    let width = COMMANDS
        .iter()
        .map(|&(f, a, _)| visible(f, a))
        .max()
        .unwrap_or(0);
    for &(flags, args, desc) in COMMANDS {
        write!(out, "    ")?;
        paint(out, Some(Color::Cyan), false, flags)?;
        if !args.is_empty() {
            write!(out, " ")?;
            paint(out, Some(Color::Magenta), false, args)?;
        }
        writeln!(
            out,
            "{:pad$}  {desc}",
            "",
            pad = width - visible(flags, args)
        )?;
    }

    writeln!(
        out,
        "\nIds are shown by --list. Wrap text that contains spaces in quotes.\n"
    )?;

    heading(out, "EXAMPLES:")?;
    for example in EXAMPLES {
        write!(out, "    ")?;
        paint(out, None, true, NAME)?;
        writeln!(out, " {example}")?;
    }
    Ok(())
}
