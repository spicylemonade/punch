<div id="top"></div>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->


<br />
<div align="center">
  <a href="https://github.com/spicylemonade/punch/">
    <img src="https://user-images.githubusercontent.com/84095175/179336378-98179393-8c64-4176-9520-afa3747de43e.png" alt="Logo" width="100" height="100">
  </a>
  <h1 align="center">punch</h1>

  <h6 align="center">
    🗂️ file and folder management written in rust :page_facing_up:

  </h6>
</div>





### About punch :bulb:

It can be kind of easy to forget which bash command does what, for folder creation, we have `mkdir`, for files `cat` and `touch` , and for deletion we have `rm`,
each of them has its own flags and ways to append multiple, so why not just have one command do it all.


Name inspired by the `touch` command

## commands:

#### :ledger: NOTE: all commands can't be joined together any number of times<br>
* ex: ```punch <flag(optional)> <file or directory_name>  <file or directory_name> ...```
* ```sh
  punch ./folder1/ file1.txt file2.txt ./folder2 ./folder3 "to create multiple files and folders"
  ```

#### all directories must start with "./" and end with "/"

* ```sh
  punch --help "to bring up help"
  ```
* ```sh
  punch <file_name>... "to create file" or directory
  ```
* ```sh
  punch -r <file1> <file2> "to rename a file"
  ```
* ```sh
  punch -d <file_name> "or"  punch -d ./<directory_name>/ "to delete"
  ```
* :bell:(bonus)

* ```sh
  punch -in ./<target_directory_name>/ <file or directory_name> "creates files inside target directory"
  ```
* ```sh
  punch -din ./<target_directory_name>/ <file or directory_name> "deletes files inside target directory"
  ```
* ```sh
  punch -t <file or directory_name> "trashed the specified file or directory"
  ```
* ```sh
  punch -m <file names separated by spaces e.g. test.txt test1.txt> <./<directory_name>/ or number_of_moves: number>
  ```
* ```sh
  punch -l "Lists the sub-directories and files in the current working directory"
  ```
* ```sh
  punch -o "opens file with default application"
  ```
* ```sh
  punch -u "undoes the last create or trash command"
  ```
* ```sh
  punch --sizeof "returns the size of a file/folder"
  ```
* ```sh
  punch -s "displays a table of file creation/deletion/trash history"
  ```

### Built With

![Rust](https://img.shields.io/badge/-Rust-000000?style=for-the-badge&logo=Rust)
![Bash](https://img.shields.io/badge/-Bash-000000?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNi1jMTExIDc5LjE1ODMyNSwgMjAxNS8wOS8xMC0wMToxMDoyMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTUgKFdpbmRvd3MpIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOkFGMDEwMjEwQUZCMzExRTVBQTQzRTE3MTg4NzYwMjc5IiB4bXBNTTpEb2N1bWVudElEPSJ4bXAuZGlkOkFGMDEwMjExQUZCMzExRTVBQTQzRTE3MTg4NzYwMjc5Ij4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6QUYwMTAyMEVBRkIzMTFFNUFBNDNFMTcxODg3NjAyNzkiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6QUYwMTAyMEZBRkIzMTFFNUFBNDNFMTcxODg3NjAyNzkiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz6VJ7djAAACPklEQVR42mJ4+/Ytg5SUFAMaYNbU0StetHTFu6UrVn3Q0TeqAIqxIisQFRVlePHiBQPDu3fvUAzg5uW3LymvOvjs+Yv/MPDi1ev/FdV1R3n5BZ2RDXj58iXEAFEREZCYnG9A8NyzFy7+xwUuXr76PzAkfCFQrSI/Pz/EABBhZGIWuHjZitf/iQSbt25/a2FlE/748WMGhsePn3Cu27D57n8SwO59B/6nZOU/efb8OQ8LIxMTt5SUJDcDEeDWnbsMXX2TGLbt3MMgIyXJxcHBwcMCFP//9+/ff/g0/vjxk2HW/IUMs+ctYvj48SMDNzcXAysr6z+Qa1gI2Xri1BmGhtZOhktXrjLwcHMz8PDwMPz+8wcuj9eAl69eM2TkFTN8+fKFQVxMjOH79+8MQEtR1DDhM+D9+/fAaH7PEOTvw+Dl7sLw89cvDDUgAxiZmZmxG8TIyMDPz8dga23JcP/BIwYWJmZggDHC5MB6mf7/+/ftxctX37HrZ2T4+fMnw7YduxmC/HwYmNgYGDik/oDFgcZ8B3rnKxMbK8u3ttaW8tXrNnzAMAHkX6BidTVVhoOHjzOwSPxkYOFgYvj/59+nN88fl//59eszOCmLQJKyckh41JLLV6/DE8yNm7f+q+ub/XfzDf5vZOX4X9PA8r+sguYKNg5ONWFhIUReQM5M/ILCbtV1jSfevn////GTp/81DS3+q+ia/FfU0DsNlPPEyEw4sjO7oYl5xfRZc9+b27l8FJWUrQUGNAe27AwQYADtqmuiowRmJAAAAABJRU5ErkJggg==
)
## Downloading ⬇️


1. Download zip corresponding release for your os (linux-punch.zip, windows-punch.zip)
2. Extract the zip file into home directory

#### Linux:

* go into folder and run ``source config.sh`` or ``. ./config.sh``

#### Windows:

* rename the folder to ".punch" then go into the folder and run ``.\config.ps1``

## Building from Source :gift:


1. Clone the repo
   ```sh
   git clone https://github.com/spicylemonade/punch.git
   ```
3. CD into the project
   ```sh
   cd ./punch
   ```
4. Run the build script
   ```sh
   source build.sh
   ```
   ##### or
   ```sh
    . ./build.sh
   ```
   ### For Windows ->
   ```sh
    ./build.ps1
   ```
#### If the build ends in an error you may not have the c libraries installed:

for debian based systems ->
```sh
  sudo apt-get install build-essential
```
for arch based
```sh
  sudo pacman -Sy base-devel
```
for windows ->

run .\vs_BuildTools.exe in the punch clone directory then click on "Desktop Development with c++" and install

<p align="right">{<a href="#top">To top</a>}</p>



<!-- EXAMPLES -->
## Examples

![image](https://user-images.githubusercontent.com/84095175/181157889-540acdb9-6f5f-4924-be9e-91e3db5da13d.png)

![image](https://user-images.githubusercontent.com/84095175/181195279-9551efa6-133e-464c-9b2a-a3dec1339086.png)




<p align="right">{<a href="#top">To top</a>}</p>

<!-- CONTRIBUTIONS -->
## contributions:eyes:

<p> contributions are welcome, just fork and pull request </p>




