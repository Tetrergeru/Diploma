FROM texlive/texlive

ADD ./* /work/

WORKDIR /work 

RUN pdflatex "arutyunov - thesis.tex"