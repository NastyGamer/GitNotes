# GitNotes

## Keep your notes written in LaTeX in the VCS

Using GitNotes you can save all your notes in a git-capable VCS (GitGub, Bitbucket,...)  

## Functionality  

All your notes live in a folder called the root.  
In this root, all you subjects are located as separate repos.  
In each of these repos, your actual notes live. These notes are submodules of the repos.

## Example folder structure

```bash
.                                   <- the root
├── CS                              <- example repo
│   ├── exam-solutions              <- example document
│   │   └── src
│   │       ├── main.tex <- the actual notes
│   │       ├── solarized-dark.sty
│   │       └── solarized.sty
│   └── introduction                <- another example document
│       └── src
│           ├── main.tex            <- more notes
│           ├── solarized-dark.sty
│           └── solarized.sty
└── git_notes

```
