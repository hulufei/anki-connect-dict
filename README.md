# Anki Dictionary.app Helper

This small tool help adding vocabulary from **Dictionary.app** as Anki card more convenient.

## Requirement

- [Anki-Connect](https://foosoft.net/projects/anki-connect/)
- Make sure Anki is open

To copy from **Dictionary.app** with styles, first open **TextEdit.app**, click
`Services > Services Preferences`, check `New TextEdit With Selected Contents`,
assign a shortcut like `Command + Alt + t`.

## Usage

Now, you can open **Dictionary.app**, query some words, select the definition
or just press `Command + a` to select all, then press `Command + Alt + t` will
open the **TextEdit.app** with the selected contents and reserved styles. Press
`Command + s` to save the contents as html file and use the query word as filename.

Run this tool: `cargo run -- html_file` will open Anki add card GUI filled with
contents.

There is also a watch mode: `cargo run -- dir` will watch `dir`, once a new file
added to `dir`, add card trigger automatically. Under watch mode, processed html
file will move to an `imported/` sub directory.

The last step is to add card styling in Anki, you can tweak yourself from html 
file, or just use mine:

```css
.card {
    font-family: arial;
    font-size: 20px;
    text-align: center;
    color: black;
    background-color: white;
}
p.p1 {margin: 0 0 0 0; font: 16.1px 'Helvetica Neue'; color: #000000; color: rgba(0, 0, 0, 0.5); -webkit-text-stroke: rgba(0, 0, 0, 0.5)}
p.p2 {margin: 0 0 0 0; font: 16.1px 'Helvetica Neue'; -webkit-text-stroke: #000000}
p.p3 {margin: 0 0 0 0; font: 13.4px 'Helvetica Neue'; -webkit-text-stroke: #000000}
p.p4 {margin: 0 0 0 0; font: 12.6px 'Helvetica Neue'; -webkit-text-stroke: #000000}
p.p5 {margin: 0 0 0 0; font: 12.1px 'Helvetica Neue'; color: #000000; color: rgba(0, 0, 0, 0.5); -webkit-text-stroke: rgba(0, 0, 0, 0.5)}
p.p6 {margin: 0 0 0 0; font: 15.2px 'Helvetica Neue'; color: #000000; color: rgba(0, 0, 0, 0.5); -webkit-text-stroke: rgba(0, 0, 0, 0.5)}
p.p7 {margin: 0 0 0 0; font: 22.8px 'Helvetica Neue'; -webkit-text-stroke: #000000}
span.s1 {font: 22.8px 'Helvetica Neue'; font-kerning: none; color: #000000; -webkit-text-stroke: 0px #000000}
span.s2 {font: 16.0px 'Helvetica Neue'; font-kerning: none}
span.s3 {font-kerning: none}
span.s4 {font: 12.6px 'Helvetica Neue'; font-kerning: none; color: rgba(0, 0, 0, 0.5); -webkit-text-stroke: 0px rgba(0, 0, 0, 0.5)}
span.s5 {font-kerning: none; color: rgba(0, 0, 0, 0.5); -webkit-text-stroke: 0px rgba(0, 0, 0, 0.5)}
span.s6 {font: 13.4px 'Helvetica Neue'; font-kerning: none; color: rgba(0, 0, 0, 0.5); -webkit-text-stroke: 0px rgba(0, 0, 0, 0.5)}
span.s7 {font: 13.4px 'Helvetica Neue'; font-kerning: none}
span.s8 {font: 16.1px 'Helvetica Neue'; font-kerning: none; color: #000000; -webkit-text-stroke: 0px #000000}
span.s9 {font: 16.1px 'Helvetica Neue'; font-kerning: none; color: rgba(0, 0, 0, 0.5); -webkit-text-stroke: 0px rgba(0, 0, 0, 0.5)}

.front { font-size: 1.5em; }
.back { text-align: left; }
.context { text-align: left; }
p.p1 { font-size: 1em; margin: 0.4em 0; }
p.p2 { font-size: 1em; margin: 0.2em 0; }
p.p3 { font-size: 0.85em; margin: 0.3em 0 0.3em 0.7em; }
p.p4 { font-size: 0.8em; margin: 0.4em 0; }
p.p5 { font-size: 0.75em; margin-top: 1em; border-bottom: 1px dashed #ccc; padding-bottom: 0.2em; }
p.p6 { font-size: 1em; margin: 0.2em 0; }
p.p7 { font-size: 1.375em; margin: 0.5em 0; }
span.s1 { font-size: 1.375em; }
span.s2, span.s8, span.s9 { font-size: 1em; }
span.s4 { font-size: 0.8em; }
span.s6, span.s7 { font-size: 0.85em; }
```
