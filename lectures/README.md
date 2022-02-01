# Lectures

This folder contains lecture presentations and materials.

You can build lecture presentations from source by the following command (First, you'll need to set up `minted` package for Latex):

```bash
$ cd lecture-XX
$ xelatex -shell-escape -synctex=1 -interaction=nonstopmode slides.tex
```

Or just open already compiled PDF called `slides.pdf`.
