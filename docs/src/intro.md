# Introduction

Welcome to *The Calypso Book*! This book is the definitive reference to the Calypso programming language.

The book will be is divided into 2 major sections:
- [The Calypso Guide](guide/intro.md), a guide on how to write code in Calypso.
- [The Calypso Reference](reference/intro.md), a more formal spec on how the Calypso programming language works. This section is inspired by the Rust reference, which has definitely gotten me through some tough binds when working on stuff with macros.

## Notation

Some chapters will use notation that is specific to that part of the book. Notation localized to a specific part of the book will be discussed when needed. However, there is some notation that will be used throughout the book. This will be described here.

### Linking/Referencing Sections

When referencing a part of the book, use the following format:
- Prefix the section number by the short name (Reference: REF, Guide: GUI) of the part of the book followed by a space
- Section numbers should be formatted as they are shown in the sidebar of the book, not including the first part (as that is dividing between the Guide and the Reference) An example of this would be `12.5.6`, if, for example, there was a section `2.12.5.6: Some Section` in the reference.
- If the referenced section is an appendix, use the appendix letter prefixed by "Appendix" instead of the section number.

#### Examples

- Reference Chapter 2, Section 3: `REF 2.3`
- Guide Chapter 7, Section 3, Subsection 5: `GUI 7.3.5`
- Guide Appendix B: `GUI Appendix B`