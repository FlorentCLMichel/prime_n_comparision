Param(
    [string]$Mode,
    [int]$NPrimes = 300000,
    [int]$NSegs = 5
)

function Run-Rust {
    cargo run --release -- $NPrimes $NSegs
}

function Run-Python {
    python3 ./prime_n.py $NPrimes $NSegs
}

function Run-Zig {
    Push-Location ./zig
    zig build-exe -lc -O ReleaseFast prime.zig
    ./prime.exe $NPrimes $NSegs
    Pop-Location
}

function Check-Op {
    param([string]$Target)
    python3 ./valid.py $Target
}

switch ($Mode) {

    "python" {
        Write-Host "Running Python"
        Run-Python
        Check-Op "python"
    }

    "rust" {
        Write-Host "Running Rust"
        Run-Rust
        Check-Op "rust"
    }

    "zig" {
        Write-Host "Running Zig"
        Run-Zig
        Check-Op "zig"
    }

    "all" {
        Write-Host "Running Python, Rust, Zig in parallel"

        Start-Job { python3 ./prime_n.py $using:NPrimes $using:NSegs } | Out-Null
        Start-Job { cargo run --release -- $using:NPrimes $using:NSegs } | Out-Null
        Start-Job {
            Push-Location ./zig
            zig build-exe -lc -O ReleaseFast prime.zig
            ./prime.exe $using:NPrimes $using:NSegs
            Pop-Location
        } | Out-Null

        Wait-Job *
        Receive-Job *
        Remove-Job *

        Check-Op ""
    }
}
