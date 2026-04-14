use clap::Parser;
use windows::Win32::System::Power::{IsPwrHibernateAllowed, IsPwrSuspendAllowed, SetSuspendState};

#[derive(Parser)]
#[command(about = "Power state manager")]
struct Args {
    /// Action: sleep or hibernate
    action: String,

    /// Force critical suspension (bForce)
    #[arg(short, long)]
    force: bool,

    /// Disable wake events (bWakeupEventsDisabled)
    #[arg(short = 'd', long)]
    disable_wake: bool,
}

fn main() {
    let args = Args::parse();

    let hibernate = match args.action.as_str() {
        "hibernate" => true,
        "sleep" => false,
        _ => {
            eprintln!("Invalid action. Use 'sleep' or 'hibernate'.");
            return;
        }
    };

    unsafe {
        if !args.force {
            if hibernate {
                if !IsPwrHibernateAllowed() {
                    eprintln!("Hibernate is not allowed.");
                    eprintln!("Run as administrator and or use --force.");
                    return;
                }
            } else {
                if !IsPwrSuspendAllowed() {
                    eprintln!("Sleep is not allowed.");
                    eprintln!("Run as administrator and or use --force.");
                    return;
                }
            }
        }

        // SetSuspendState(bHibernate, bForce, bWakeupEventsDisabled)
        let result = SetSuspendState(hibernate, args.force, args.disable_wake);

        if !result {
            eprintln!("Failed to change power state.");
        }
    }
}
