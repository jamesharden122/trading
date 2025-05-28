from .ieee754 import IEEE754Processor
from tensor import Tensor, TensorSpec, TensorShape
from utils.index import Index

struct float64_column:
    var name: String
    var file_path: String
    fn __init__(out self, name: String, file_path: String):
        self.name = name
        self.file_path = file_path

    fn bytes_to_float(self) -> Tensor[DType.float64]:
        try:
            print(self.file_path)
            var file =open(self.file_path,"r")
            var buffer = file.read_bytes(-1)
            file.close()
            var list_simd = List[SIMD[DType.uint8,8]]()
            for i in range(0,len(buffer),8):
                var temp_simd = SIMD[DType.uint8,8](
                    Int(buffer[i]),
                    Int(buffer[i+1]),
                    Int(buffer[i+2]),
                    Int(buffer[i+3]),
                    Int(buffer[i+4]),
                    Int(buffer[i+5]),
                    Int(buffer[i+6]),
                    Int(buffer[i+7])
                )
                list_simd.append(temp_simd)
            var float_struct = IEEE754Processor(list_simd)
            var data = float_struct.float_64_conversion()
            return(data)
        except:
            print("Float Error")
            return(Tensor[DType.float64]())

