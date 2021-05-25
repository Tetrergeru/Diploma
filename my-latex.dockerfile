FROM texlive/texlive

ADD ./* .

RUN pdflatex "arutyunov - thesis.tex"