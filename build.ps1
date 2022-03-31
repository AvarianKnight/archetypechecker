Invoke-Expression -Command "cargo build --release"

if (Test-Path -Path ".\archetypechecker.exe") {
    Remove-Item -Path ".\archetypechecker.exe"
}

if (Test-Path -Path ".\target\release\archetypechecker.exe") {
    Move-Item -Path ".\target\release\archetypechecker.exe" -Destination ".\"
}