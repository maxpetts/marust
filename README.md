# Must (M~~arkdown~~ ~~r~~UST)

A converter for your markdown files 📁 => 📂

---

**MVP**: Convert markdown files into html tags

**End Goal**: Convert markdown files into html & pdf, constructing the pdfs from scratch

## Syntax support:

Following [Markdown Guide](https://www.markdownguide.org/basic-syntax/#paragraphs-1)

- [x] Headings (#)
- [x] Paragraphs (NADDA)
- [ ] Line Breaks (is it a paragraph? replace new line with \<br> tag)
- [ ] Emphasis (\*)
  - [x] bold (\*\*blah\*\* OR \_\_blah\_\_)
  - [x] italic (\*blah\* OR \_blah\_)
  - [ ] underline (\*\*\*blah**_ OR \_\_\_blah\_\_\_ OR \_\_\*blah_\__ OR \*\*\_blah_**)
  - [x] cross-through
- [ ] Blockquotes (line prefaced by \>) -> support indents for nested
- [ ] Lists -> support indents for nested
  - [ ] Unordered (- OR \* OR +)
  - [x] Ordered (numbered lists)
- [ ] Code
  - [ ] Inline (\`)
  - [ ] Block (prepended by 4 spaces)
- [ ] Horizontal Rules (on own line & >3 \*, \_ or -)
- [ ] Links (complex figure out later)
- [ ] Images (a link prepended by !)
- [ ] Escaping Characters
- [ ] Math
- [ ] Inline html
  - [ ] XSS protection? etc; but images..
- [ ] Handle escape characters (\\)

## Similar projects

- Python-Markdown : https://github.com/Python-Markdown/markdown
