# Rumbaa - RUst Mathematics Bloc Analysis for lAtex document
[![Build Status](https://travis-ci.com/c-elvira/rumbaa.svg?token=rHHx69ioGqz4NFraNjyT&branch=master)](https://travis-ci.com/c-elvira/rumbaa)

**Rumbaa** is a theorem analyzer for latex documents written in Rust.
With Tex files as input, **Rumbaa** outputs a graph displaying the dependencies between mathematical structures (*e.g.*, theorems, lemmas etc...)

![example](docs/struct_example.png)

The above example shows has been obtained from:
	- **When does OMP achieves support recovery with continuous dictionaries?** by Clément Elvira, Rémi Gribonval, Charles Soussen, and Cédric Herzet, 2019 

## Installation - *Work in progress*

1. Make sure that Rust is available on your computer. Otherwise see [this link](https://www.rust-lang.org/tools/install).

2. Download repository
```
git clone https://github.com/c-elvira/rumbaa.git
```

3. Run the installation script
```
./install.sh
```

## Usage

1. Formating Latex document

**Latex environments:** Rumbaa parses Latex documents by looking after the usual latex structures such as *Theorem*, *Proposition*, etc.
Your latex document should look like this:
``` latex
\newtheorem{theorem}{Theorem}[section]
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{proposition}[theorem]{Proposition}
\newtheorem{corollary}[theorem]{Corollary}

\begin{theorem}
    \label{th:my_label}
    ...
\end{theorem}
```
To avoid issues, the file should compiled without warning and all structures should be labeled.



**Proofs:**
In order to create links between mathematical structures, Rumbaa also parses proofs.
Since a proof may not be right after a result, I recommend adding the following *latexmk*-like option in the proof environment:
``` latex
\begin{proof}
    %!TEX proof = {th:my_label}
    ...
\end{proof}
```


**Nested documents:** Rumbaa can handle nested latex files. For now, only `\input{file}` and `\include{file}` are supported.


**Auxiliary files:** By default, Rumbaa identifies a mathematical structure by its label.
If the auxiliary files produced by latex (.aux) are found, Rumbaa will also parse them to improve visualization.

2. Terminal

In your terminal, call
```
    rumbaa my_document.tex
```
The outputs are
 * A html file called *index.html*. open it to see the mathematical structure of your document,
 * a json files containing the detected structures and links.

Options are:
 * -f, --folder: if the main latex file is not in the current directory,
 * -o, --output: to specify the output directory, 
 * -a, --aux: to specify the directory containing auxiliary files (may improve visualization),
 * --arxiv to keep a clean and all in one document.


## Milestones

 - [x] Fist prototype: parse a multi-files latex document and display 
 - [x] Use equation label to improve connections between mathematical structures
 - [x] Use custom structures for latex (others than basic ones)
 - [ ] Improve output
 - [ ] Generate report
 - [x] Unit testing


## Work in progress

This is still a work in progress so there's still bugs to iron out. As this is my first project in Rust the code could no doubt use an increase in quality but I'll be improving on it whenever I find the time. If you have any feedback feel free to raise an issue/submit a PR.

## Alternatives

If you known any alternative to Rumbaa feel free to raise issue/submit a PR or send me a mail.

## License

rumbaa is licensed under the [MIT License](https://opensource.org/licenses/MIT).

