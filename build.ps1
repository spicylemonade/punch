if ((Get-Command rustup -errorAction SilentlyContinue)){
    wget https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe `
    -outfile "rustup-init.exe"
    
    , "1" * 2 | ./rustup-init.exe

}


if (!(Test-Path -Path "$Env:USERPROFILE\.punch")){
        mkdir -p "$Env:USERPROFILE\.punch\trash"

        mkdir "$Env:USERPROFILE\.punch\bin"

        echo "~/.punch created in home" 
}
cargo build --release

mv .\target\release\punch.exe $Env:USERPROFILE\.punch\bin\


$Env:PATH = "$Env:PATH; $ENV:USERPROFILE\.punch\bin\"

$Env:PATH >> safe.txt

setx PATH "$Env:PATH; $ENV:USERPROFILE\.punch\bin\" -m

echo "build complete-- type: punch -h"