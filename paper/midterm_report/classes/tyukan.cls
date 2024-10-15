%% tyukan.cls
%% $Date: 2010/07/03 09:50:06 $
%%
%% MINOURA Hirohito

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{tyukan}
\ProcessOptions
\LoadClass[twoside,11pt,a4paper]{jarticle}
%% 用紙サイズ (A4)
\setlength{\paperwidth}{210mm}
\setlength{\paperheight}{297mm}
\@settopoint{\paperwidth}
\@settopoint{\paperheight}

%% 用紙左上を原点とする
\setlength{\voffset}{-1in}
\setlength{\hoffset}{-1in}

%% 傍注を削除
\setlength{\marginparsep}{0pt}
\setlength{\marginparwidth}{0pt}
\setlength{\marginparpush}{0pt}

%% 左マージン
\setlength{\oddsidemargin}{27mm}
\setlength{\evensidemargin}{27mm}
\@settopoint{\oddsidemargin}
\@settopoint{\evensidemargin}

%% 上マージン
\setlength{\headheight}{\z@}
\setlength{\headsep}{\z@}
\setlength{\topsep}{\z@}
\setlength{\topmargin}{17.5mm}
%\setlength{\bottommargin}{25mm}
\@settopoint{\topmargin}

%% 脚注領域を削除
\setlength{\footnotesep}{\z@}
\setlength{\footskip}{\z@}

%% テキスト領域
\setlength{\textwidth}{\paperwidth}
\addtolength{\textwidth}{-\oddsidemargin}
\addtolength{\textwidth}{-\evensidemargin}
\@settopoint{\textwidth}
\setlength{\textheight}{\paperheight}
\addtolength{\textheight}{-\topmargin}
%\addtolength{\textheight}{-\bottommargin}
\addtolength{\textheight}{-25mm}
\@settopoint{\textheight}

%% 文字間・行間
\kanjiskip=.12zw plus 2pt minus 2pt
\xkanjiskip=.12zw plus 2pt minus 2pt
\renewcommand{\baselinestretch}{1.1}

%% 年度
\newcommand\@year\@empty
\newcommand{\Year}[1]{\def\@year{#1}}

%% 学籍番号
\newcommand{\no}{\@empty}
\newcommand{\No}[1]{\def\@no{#1}}

%% 氏名
\newcommand{\@name}{\@empty}
\newcommand{\Name}[1]{\def\@name{#1}}

%% 研究室
\newcommand\@laboratory\empty
\newcommand{\Laboratory}[1]{\def\@laboratory{#1}}

%% 題目
\newcommand{\@theme}{\@empty}
\newcommand{\Theme}[1]{\def\@theme{#1}}

\newcommand{\@keywords}{\@empty}
\newcommand{\Keywords}[1]{\def\@keywords{#1}}

%% 内容
\newcommand{\@contents}\@empty
\newcommand{\Contents}[1]{\def\@contents{#1}}

\renewcommand{\paragraph}{\@startsection{paragraph}{4}{\z@}%
   {0ex \@plus 1ex \@minus .2ex}%
   {-0.1em}%
   {\reset@font\normalsize\bfseries}}

%% 予稿を作成するコマンド．\end{document} で実行される．
\newcommand{\@maketyukan}{%
%% picture 環境
\noindent
\setlength{\unitlength}{1truemm}
\begin{picture}(160, 254)(3, 0)
%% ヘッダ
\thinlines
\put(0, 0){\framebox(160,254){}}
\put(0, 248){\makebox(160,6)[r]{\small \@year 年度電子情報工学科　卒業研究中間報告\ }}
\put(0, 248){\line(1,0){160}}
\put(0, 242){\makebox(29,6)[c]{報告者}}
\put(29, 248){\line(0,-1){32}}
\put(29, 242){\makebox(17.5,6)[c]{\@no 席}}
\put(46.5, 248){\line(0,-1){6}}
\put(46.5, 242){\makebox(17.5,6)[c]{氏名}}
\put(64, 248){\line(0,-1){6}}
\put(64, 242){\makebox(45,6)[c]{\textbf{\@name}}}
\put(109, 248){\line(0,-1){6}}
\put(109, 242){\makebox(20,6)[c]{研究室}}
\put(129, 248){\line(0,-1){6}}
\put(129, 242){\makebox(29,6)[c]{\textbf{\@laboratory}}}
\put(0, 242){\line(1,0){160}}
\put(0, 223.5){\makebox(29,18.5)[c]{論文題目}}
\put(29, 223.5){\makebox(131,18.5)[c]{\parbox{127mm}{\textbf{\@theme}}}}
\put(0, 223.5){\line(1,0){160}}
\put(0, 216){\line(1,0){160}}
\put(0, 216){\makebox(29,7.5)[c]{キーワード}}
\put(31, 216){\makebox(131,7.5)[l]{\textbf{\@keywords}}}

%% 本文
%\put(0,0){\makebox(160, 216){\parbox[l][210mm][t]{156mm}{\@contents}}}
\end{picture}
\vspace*{-222mm}
}
%% フックマクロ定義コマンド
\AtBeginDocument{\pagestyle{empty}}
\AtBeginDocument{\@maketyukan}
%\AtEndDocument{\@maketyukan}

\endinput
