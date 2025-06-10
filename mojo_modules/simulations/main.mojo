from python import Python
from max.graph import Graph, TensorType, ops
from max.tensor import Tensor, TensorShape
from time import perf_counter
from monte_carlo import BrownianBridge, GeomBM
from max.graph.checkpoint import load, save, TensorDict
from read_bin import DfReader
from collections import Dict

def main():
    #Read Data
    base_path = "/home/yakaman/trading/mojo_modules/simulations/"
    var bin_paths = Dict[String,List[String]]()
    bin_paths["uint64"] = List[String]("ts_recv.bin")
    bin_paths["uint32"] = List[String]("ts_in_delta.bin", "size.bin")#"instrument_id.bin"
    bin_paths["uint8"] = List[String]()#"flags.bin","depth.bin"
    bin_paths["int64"] = List[String]("price.bin")
    bin_paths["int32"] = List[String]("ts_in_delta.bin")
    bin_paths["f64"] = List[String]()
    var df = DfReader(bin_paths,base_path)
    var tens_dict = df.create()
    
    #Simulation
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
