# Analysing Query Results with Jupyter

Jupyter notebook is a web application commonly used by data scientists to analyse and visualise their data. If you have Docker installed, you can start a local Jupyter instance as follows:

```bash
make run-jupyter
```

*Note:* Since using Docker requires root permissions, this command will ask for `sudo` password.

The command will print to the terminal a message like this:

```plain
[C 10:33:17.685 NotebookApp]

    To access the notebook, open this file in a browser:
        file:///home/jovyan/.local/share/jupyter/runtime/nbserver-27-open.html
    Or copy and paste one of these URLs:
        https://4ad49d6251da:8888/?token=202176e7bd7283e90ba6321c58472d193f41e27ba0da2b41
     or https://127.0.0.1:8888/?token=202176e7bd7283e90ba6321c58472d193f41e27ba0da2b41

```

Click on one of the links to open the notebook in your default browser. The notebook uses a self-signed certificate and, as a result, your browser will show an SSL error. Just ignore it.

If everything started successfully, you should see three folders listed: `data`, `reports`, and `work`. Click on `reports`. It should contain six files with `.ipynb` extensions–these are Python notebooks used to analyse the data presented in the paper.

After you open a notebook (for example, by clicking on `Builds.ipynb`), you can re-execute it by choosing *Kernel* → *Restart & Run All*.
