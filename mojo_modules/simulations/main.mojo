from python import Python
from max.graph import Graph, TensorType, ops
from max.tensor import Tensor, TensorShape
from time import perf_counter
from monte_carlo import BrownianBridge, GeomBM
from max.graph.checkpoint import load, save, TensorDict


def main():
    var brown_brdg = BrownianBridge(24,1, TensorDict())
    var out_tensor = brown_brdg.weiner_mgf_1d(0.0,brown_brdg.generate_bridge())
    var geom = GeomBM(5,1,24,0.07,0.6, TensorDict(), 15.0)
    try:
        var t1 = perf_counter()                                            
        var sim = geom.gen_mcs_lazy()
        var t2 = perf_counter()
        var sim_time = t2 - t1
        print(sim_time)
        print(sim)
    except:
        print("error")     
