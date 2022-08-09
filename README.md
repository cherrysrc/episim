<h1 align="center">Episim</h1>
<p align="center">Simulating epidemics</p>

<h2>Configuration</h2>
<p>Each configuration has its own folder consisting of two files. Check the example configurations if in doubt.</p>
<ul>
    <li>core.cfg. Contains various simulation parameters.</li>
    <li>demographic.csv. Determines the demographic composition of the simulations population. CSV files for countries can be downloaded at <a href="https://www.populationpyramid.net/">populationpyramid.net</a></li>
    <li>(Source code). Toggle the live visualisation by uncommenting the one you want to use (<a href="https://github.com/cherrysrc/episim/blob/master/src/main.rs#L38">main.rs</a>). Keep in mind the graphic version will take longer to run.</li>
    <li>(Source code). You can adjust the source code to adjust the simulation parameters. Some parameters, such as the functions computing the infection and survival chances, are easily accessible via the static config object in <a href="https://github.com/cherrysrc/episim/blob/master/src/main.rs#L17">main.rs</a>. Serializing these could be a future improvement.</li>
</ul>

<h2>Gallery</h2>
<p align="center">
    <img src="export/example_conf_2022-08-09_13-05-53/demographics.png" width=350></img>
    <img src="export/example_conf_2022-08-09_13-05-53/trend.png" width=350></img>
</p>

<p>Higher test rate</p>
<p align="center">
    <img src="export/example_conf_2022-08-09_13-09-16/demographics.png" width=350></img>
    <img src="export/example_conf_2022-08-09_13-09-16/trend.png" width=350></img>
</p>

<p>Shorter recovered period</p>
<p align="center">
    <img src="export/example_conf_2022-08-09_13-10-04/demographics.png" width=350></img>
    <img src="export/example_conf_2022-08-09_13-10-04/trend.png" width=350></img>
</p>

<p>Pyramid demographic</p>
<p align="center">
    <img src="export/other_example_conf_2022-08-09_13-07-35/demographics.png" width=350></img>
    <img src="export/other_example_conf_2022-08-09_13-07-35/trend.png" width=350></img>
</p>

<p>Without reinfection</p>
<p align="center">
    <img src="export/other_example_conf_2022-08-09_19-39-55/demographics.png" width=350></img>
    <img src="export/other_example_conf_2022-08-09_19-39-55/trend.png" width=350></img>
</p>
