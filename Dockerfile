# FROM texlive/texlive

ADD ./* /work/

WORKDIR /work 

RUN touch "arutyunov - thesis.tex"