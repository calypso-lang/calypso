// highlight.js syntax highlighting for Calypso

const calypso = (hljs) => {
  let KEYWORDS = 'is if else for in loop while match ' +
                 'ret break fn native mod use import pub ' +
                 'let mut del as';

  let BUILTINS = "println typeof bool sint uint float" +
                 "string char tuple array";
    
  return {
    name: 'Calypso',
    aliases: ['cal'],
    keywords: {
      $pattern: hljs.IDENT_RE + '!?',
      keyword: KEYWORDS,
      literal: 'true false null undef',
      built_in: BUILTINS
    },
    illegal: "</",
    contains: [
      hljs.C_LINE_COMMENT_MODE,
      hljs.COMMENT('/\\*', '\\*/', {contains: ['self']}),
      hljs.QUOTE_STRING_MODE,
      {
        className: 'string',
        begin: /'\\?(x\w{2}|u\w{4}|U\w{8}|.)'/
      },
      {
        className: 'number',
        variants: [
          { begin: '\\b0b([01_]+)([ufs]?)' },
          { begin: '\\b0o([0-7_]+)([ufs]?)' },
          { begin: '\\b0x([A-Fa-f0-9_]+)([ufs]?)' },
          { begin: '\\b(\\d[\\d_]*(\\.[0-9_]+)?([eE][+-]?[0-9_]+)?)([ufs]?)'}
        ],
        relevance: 0
      },
      {
        className: 'function',
        beginKeywords: 'fn', end: '\\(', excludeEnd: true,
        contains: [hljs.UNDERSCORE_TITLE_MODE]
      },
      {
        className: 'meta',
        begin: '#!?\\[', end: '\\]',
        contains: [
          {
            className: 'meta-string',
            begin: /"/, end: /"/
          }
        ]
      }
    ]
  }
};

hljs.registerLanguage("calypso", calypso);
hljs.registerAliases("cal", {languageName: "calypso"});