from max.tensor import Tensor, TensorSpec, TensorShape
from max.graph.checkpoint import load, save, TensorDict
from collections import Dict
from bin_to_df import DfReader

def main():
    base_path = "/home/yakaman/Desktop/trading/bento_queries/"
    var bin_paths = Dict[String,List[String]]()
    bin_paths["uint64"] = List[String]("ts_recv.bin")
    bin_paths["uint32"] = List[String]("ts_in_delta.bin", "size.bin")#"instrument_id.bin"
    bin_paths["uint8"] = List[String]()#"flags.bin","depth.bin"
    bin_paths["int64"] = List[String]("price.bin")
    bin_paths["int32"] = List[String]("ts_in_delta.bin")
    bin_paths["f64"] = List[String]()
    var df = DfReader(bin_paths,base_path)
    var tens_dict = df.create()
    
