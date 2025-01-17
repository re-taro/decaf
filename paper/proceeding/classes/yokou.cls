%
%  This class file is used for making manuscript style.
%    Ver 1.00  1997.05.18
%    Ver 1.01  1997.05.22
%    Ver 1.02  1997.06.04
%    Ver 1.03  1999.05.27
%   by hagihira@masui.med.osaka-u.ac.jp
%      Department of Anesthesiology and Surgical Center
%      Osaka Prefectural Habikino Hospital,
%      3-7-1 Habikino, Habikino City, Osaka Japan 583-8588.
%
%
%  Modified by Minoura
%
\ProvidesClass{yokou}[2008/01/17]
\LoadClassWithOptions{jarticle}
\usepackage{newcent}

%
%  double space setting
%
\setlength{\paperwidth}{210mm}
\setlength{\paperheight}{297mm}
\setlength{\oddsidemargin}{-5mm}
\setlength{\evensidemargin}{-5mm}
\setlength{\topmargin}{-5mm}
\setlength{\headheight}{0mm}
\setlength{\headsep}{0mm}
\setlength{\footskip}{15mm}
\setlength{\textheight}{251.6mm}
\setlength{\textwidth}{169.2mm}
\setlength{\columnsep}{10mm}

\if@titlepage
\renewcommand{\maketitle}{\begin{titlepage}%
    \let\footnotesize\small
    \let\footnoterule\relax
    \let\footnote\thanks
    \null\vfil
    \vskip 60\p@
    \begin{center}%
      {\LARGE \@title \par}%
      \vskip 3em%
      {\Large
        \lineskip .75em%
        \begin{tabular}[t]{c}%
          \@author
        \end{tabular}\par}%
      \vskip 1.5em%
      {\large \@date \par}%       % Set date in \large size.
    \end{center}\par
    \@thanks\vfil\null
  \end{titlepage}%
  \setcounter{footnote}{0}%
  \global\let\thanks\relax
  \global\let\maketitle\relax
  \global\let\p@thanks\relax
  \global\let\@thanks\@empty
  \global\let\@author\@empty
  \global\let\@date\@empty
  \global\let\@title\@empty
  \global\let\title\relax
  \global\let\author\relax
  \global\let\date\relax
  \global\let\and\relax
}%
\else
\renewcommand{\maketitle}{\par
  \begingroup
  \renewcommand{\thefootnote}{\fnsymbol{footnote}}%
  \def\@makefnmark{\hbox{\ifydir $\m@th^{\@thefnmark}$
      \else\hbox{\yoko$\m@th^{\@thefnmark}$}\fi}}%
  \long\def\@makefntext##1{\parindent 1em\noindent
    \hbox to1.8em{\hss$\m@th^{\@thefnmark}$}##1}%
  \if@twocolumn
  \ifnum \col@number=\@ne \@maketitle
  \else \twocolumn[\@maketitle]%
  \fi
  \else
  \newpage
  \global\@topnum\z@   % Prevents figures from going at top of page.
  \@maketitle
  \fi
  \thispagestyle{empty}\@thanks
  \endgroup

  \setcounter{footnote}{0}%
  \global\let\thanks\relax
  \global\let\maketitle\relax
  \global\let\p@thanks\relax
  \global\let\@thanks\@empty
  \global\let\@author\@empty
  \global\let\@date\@empty
  \global\let\@title\@empty
  \global\let\title\relax
  \global\let\author\relax
  \global\let\date\relax
  \global\let\and\relax
  }
\def\@maketitle{
  \null
  \thispagestyle{empty}
  {\centering
    {\bf \Large \@title \par}
    {\bf \large \@author \hspace{1zw} \@laboratory \par}
    {\Large \@etitle \par}
    {\large \@eauthor \par}
  }
  \vspace{2em}
  \gdef\@laboratory{}
  \gdef\@title{}
  \gdef\@author{}
  \gdef\@etitle{}
  \gdef\@eauthor{}
}

\long\def\laboratory#1{\long\gdef\@laboratory{#1}}
\long\def\title#1{\long\gdef\@title{#1}}
\long\def\author#1{\long\gdef\@author{#1}}
\long\def\etitle#1{\long\gdef\@etitle{#1}}
\long\def\eauthor#1{\long\gdef\@eauthor{#1}}

%
%  omit_number = 0
%
%  citation_form = 4
%
%  biblio_heading = 4
%
%    symbol definition
%
\newcommand{\degree}{$^{\circ}$}
\newcommand{\degC}{{$^{\circ}$C}}
\newcommand{\degF}{{$^{\circ}$F}}
\newcommand{\Rmark}{\raisebox{1ex}{\textregistered}}

\makeatletter
\renewcommand{\section}{%
  \@startsection{section}% #1 見出し
   {1}% #2 見出しのレベル
%   {\z@}% #3 横組みの場合，見出し左の空き(インデント量)
   {\z@}
%   {1.5\Cvs \@plus.5\Cdp \@minus.2\Cdp}% #4 見出し上の空き
   {1em}% #4 見出し上の空き
%   {.5\Cvs \@plus.3\Cdp}% #5 見出し下の空き (負の値なら見出し後の空き)
   {.1\Cvs}
%  {\reset@font\Large\bfseries}% #6 見出しの属性
%  {\centering\reset@font\Large\bfseries}% 中央揃え
%  {\raggedleft\reset@font\Large\bfseries}% 右揃え
   {\reset@font\normalsize\bf}
}%
\def\thesection{\@arabic\c@section .}
\def\thesubsection{\@arabic\c@section .\@arabic\c@subsection . }
\def\thesubsubsection{\@arabic\c@section .\@arabic\c@subsection .\@arabic\c@subsubsection .}

\renewcommand{\subsection}{
  \@startsection{subsection}
  {1}{\z@}{1\Cvs}{.1\Cvs}{\reset@font\normalsize\bf}
}

\renewcommand{\subsubsection}{
  \@startsection{subsubsection}
  {1}{\z@}{1\Cvs}{.1\Cvs}{\reset@font\normalsize\bf}
}

\renewcommand{\paragraph}{
  \@startsection{paragraph}
  {1}{\z@}{1\Cvs}{-1\Cvs}{\reset@font\normalsize\bf}
}

\long\def\@makecaption#1#2{%
  \vspace{-1em}
  \vskip\abovecaptionskip
  \iftdir\sbox\@tempboxa{#1\hskip1zw#2}%
    \else\sbox\@tempboxa{#1\hskip1zw#2}%
  \fi
  \ifdim \wd\@tempboxa >\hsize
    \iftdir #1\hskip1zw#2\relax\par
      \else #1\hskip1zw#2\relax\par\fi
  \else
    \global \@minipagefalse
    \hbox to\hsize{\hfil\box\@tempboxa\hfil}%
  \fi
  \vskip\belowcaptionskip
}

\renewenvironment{itemize}%% itemize 環境を再定義
{
  \ifnum \@itemdepth >\thr@@\@toodeep\else
  \advance\@itemdepth\@ne
  \edef\@itemitem{labelitem\romannumeral\the\@itemdepth}%
  \expandafter
  \vspace{-0.7zw}
  \list{\csname \@itemitem\endcsname}{%
    \iftdir
    \ifnum \@listdepth=\@ne \topsep.5\normalbaselineskip
    \else\topsep\z@\fi
    % 元々ここにあった定義を...
    \labelwidth1zw \labelsep.3zw
    \ifnum \@itemdepth =\@ne \leftmargin1zw\relax
    \else\leftmargin\leftskip\fi
    \advance\leftmargin 1zw
    \fi
    \parskip\z@ \itemsep\z@ \parsep\z@% こちらに移動
    \def\makelabel##1{\hss\llap{##1}}}%
  \fi
}{\endlist \vspace{-0.7zw}}

\renewenvironment{description}%% description 環境を再定義
{
  \vspace{-0.7zw}
  \list{}{\labelwidth\z@ \itemindent-\leftmargin
    \iftdir
    \leftmargin\leftskip \advance\leftmargin3\Cwd
    \rightmargin\rightskip
    \labelsep=1zw% 元々ここにあった定義を...
    \fi
%    \itemsep\z@%こちらに移動
    \parskip\z@ \itemsep\z@ \parsep\z@% こちらに移動
    \listparindent\z@ \topskip\z@ \parskip\z@ \partopsep\z@% こちらに移動
    \let\makelabel\descriptionlabel}
}{\endlist \vspace{-0.7zw}}

\let\orgcenter\center%% オリジナルの center 環境を退避
\let\orgendcenter\endcenter%%
\def\center{\vspace{-.5\baselineskip}\orgcenter}%% 改変した center 環境の定義
\def\endcenter{\orgendcenter\vspace{-.5\baselineskip}}%%

\renewcommand{\@biblabel}[1]{#1)}
\renewenvironment{thebibliography}[1]%
{\section{\refname\@mkboth{\refname}{\refname}}%
   \list{\@biblabel{\@arabic\c@enumiv}}%
        {\settowidth\labelwidth{\@biblabel{#1}}%
         \leftmargin\labelwidth
         \advance\leftmargin\labelsep
         \parskip\z@ \itemsep\z@ \parsep\z@
         \@openbib@code
         \usecounter{enumiv}%
         \let\p@enumiv\@empty
         \renewcommand\theenumiv{\@arabic\c@enumiv}}%
   \sloppy
   \clubpenalty4000
   \@clubpenalty\clubpenalty
   \widowpenalty4000%
   \sfcode`\.\@m}
  {\def\@noitemerr
    {\@latex@warning{Empty `thebibliography' environment}}%
   \endlist}

\makeatother
\renewcommand{\theparagraph}{}
\pagestyle{empty}



%
% end of yokou.cls -----------------------------------------------
