if (!(Get-Command rustup -errorAction SilentlyContinue)){
    curl https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe `
    -o "rustup-init.exe"
    
    , "1" * 2 | ./rustup-init.exe

}


if (!(Test-Path -Path "$Env:USERPROFILE\.punch")){
        mkdir -p "$Env:USERPROFILE\.punch\trash"

        mkdir "$Env:USERPROFILE\.punch\bin"

        echo "~/.punch created in home" 
}
cargo build --release
curl https://aka.ms/vs/16/release/vs_BuildTools.exe `
    -o "vs_BuildTools.exe"

mv .\target\release\punch.exe $Env:USERPROFILE\.punch\bin\


$Env:PATH = "$Env:PATH; $ENV:USERPROFILE\.punch\bin\"

echo '$Env:PATH = "$Env:PATH; $ENV:USERPROFILE.punch\bin"' >> $profile


echo "build complete-- type: punch -h"