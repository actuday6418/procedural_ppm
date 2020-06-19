import os
import time
import subprocess
import signal

for i in range(200):
    os.system("./for_testing " + str(i) + " 32 32")
    ps = subprocess.Popen("xviewer output.ppm",shell = True, preexec_fn = os.setsid)
    time.sleep(1)
    os.killpg(os.getpgid(ps.pid), signal.SIGTERM)

