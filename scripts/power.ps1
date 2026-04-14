function Suspend-Computer {
    param(
        [switch]$Force,
        [switch]$DisableWake
    )

    $f = if ($Force) { 1 } else { 0 }
    $d = if ($DisableWake) { 1 } else { 0 }

    # SetSuspendState(bHibernate=0, bForce, bWakeupEventsDisabled)
    rundll32.exe powrprof.dll, SetSuspendState 0, $f, $d
}

function Stop-ComputerHibernate {
    param(
        [switch]$Force,
        [switch]$DisableWake
    )

    $f = if ($Force) { 1 } else { 0 }
    $d = if ($DisableWake) { 1 } else { 0 }

    # SetSuspendState(bHibernate=1, bForce, bWakeupEventsDisabled)
    rundll32.exe powrprof.dll, SetSuspendState 1, $f, $d
}

