# Range and print functions available as builtins
from max.tensor import Tensor, TensorShape, TensorSpec
from max.graph.checkpoint import save, TensorDict
from random import randn_float64, randn
from algorithm.functional import vectorize
from utils.index import Index
from math import log2, sqrt, exp
from algorithm import mean, variance
from random import seed, random_ui64
from time import perf_counter
#from engine import NamedTensor

def tens_dict_create( time_tensor: Tensor[DType.float64], weiner_tensor: Tensor[DType.float64]) -> TensorDict:
    var tensors = TensorDict()
    tensors.set("Time", time_tensor)
    tensors.set("Weiner",weiner_tensor)
    return(tensors)

fn gen_ones_vec(time_steps: Int) -> Tensor[DType.float64]:
    var shape = TensorShape(time_steps)
    var one_vec = Tensor[DType.float64](TensorSpec(DType.float64,shape))
    for ind in range(0,time_steps):
        one_vec[Index(ind)] = 1/time_steps
    return(one_vec)

fn gen_time_vec(time_steps: Int) -> Tensor[DType.float64]:
    var t_vec_shape = TensorShape(time_steps,2)
    var t_vec = Tensor[DType.float64](TensorSpec(DType.float64,t_vec_shape))
    var time_delta = 1/time_steps
    for ind in range(0,time_steps):
          t_vec[Index(ind,0)] = ind*time_delta
          t_vec[Index(ind,1)] = (ind*time_delta)**(1/2)
    return(t_vec)

fn gen_z_vec(time_steps: Int) -> Tensor[DType.float64]:
    var mean = SIMD[DType.float64,1](0.0)
    var stand_dev = SIMD[DType.float64,1](1.0)
    var time_delta = 1/time_steps
    print(time_delta)
    var shape = TensorShape(time_steps)
    var z_norm = randn_tensor(shape, mean, stand_dev)

    return(z_norm)

fn randn_tensor(shape: TensorShape, mean: Float64 = 0.0, std_dev: Float64 = 1.0) -> Tensor[DType.float64]:
    var spec = TensorSpec(DType.float64, shape)
    var tensor = Tensor[DType.float64](spec)
    randn[DType.float64](tensor.unsafe_ptr(), tensor.num_elements(),
                         SIMD[DType.float64, 1](mean),
                         SIMD[DType.float64, 1](std_dev))
    return tensor

struct BrownianBridge:
    var time_steps: Int
    var time_span: Int32
    var power: Int
    var tens_dict: TensorDict                                               
    
    fn __init__(out self, power: Int, time_span: Int32, tens_dict: TensorDict):
        self.power = power
        self.time_steps = 2**power
        self.time_span = time_span
        self.tens_dict = tens_dict                                         

    fn generate_bridge(mut self) -> Tensor[DType.float64]:
        seed()
        var t_vec = gen_time_vec(self.time_steps)
        var w_vec = Tensor[DType.float64](TensorSpec(DType.float64,TensorShape(self.time_steps)))
        var z_norm = gen_z_vec(self.time_steps)
        var one_vec = gen_ones_vec(self.time_steps)
        #initialize
        var h = self.time_steps
        var j_max = 1                                              
        w_vec[Index(h-1)] = t_vec[Index(h-1,1)]*z_norm[Index(h-1)]
        print("final condition:", w_vec[Index(h-1)])

        var t_0 = t_vec[Index(0,0)]
        var w_0 = 0

        for k in range(1,self.power+1):
            var i_temp = h/2
            var i_min = i_temp
            var i = i_min
            var l = 0
            var r = h
            for j in range(1,j_max+1):
              var a: Float64 = ((t_vec[Index(r-1,0)]-t_vec[Index(l,0)])*w_vec[Index(l)]+
                   (t_vec[Index(i-1,0)]-t_vec[Index(l,0)])*w_vec[Index(r-1)])/(t_vec[Index(r-1,0)] - t_vec[Index(l,0)])
              var b: Float64 = (((t_vec[Index(i-1,0)]-t_vec[Index(l,0)])*(t_vec[Index(r-1,0)]-
                                                           t_vec[Index(i-1,0)]))/(t_vec[Index(r-1,0)] - t_vec[Index(l,0)]))**(1/2)
              w_vec[Index(i-1)] = a+b*z_norm[Index(i-1)]
              #print(w_vec[Index(i-1)])
              i = i+h
              l = l+h-1
              r = r+h
            j_max = 2*j_max
            h = Int(i_min)
        try:
          self.tens_dict = tens_dict_create(t_vec,w_vec)
        except:
          print("error")
        return(w_vec)

    fn weiner_mgf_1d(self, theta: Float64, weiner: Tensor[DType.float64]) -> Tensor[DType.float64]:
        #tensor.NamedTensor("weiner",weiner)
        var weiner_len = weiner.num_elements()
        print(weiner_len)
        var mgf_mean_vec = Tensor[DType.float64](TensorSpec(DType.float64,TensorShape(weiner_len)))
        var mgf_var_vec = Tensor[DType.float64](TensorSpec(DType.float64,TensorShape(weiner_len)))
        var mean_sum: Float64 = 0
        var variance_sum: Float64 = 0
        var pos_count = 0
        for i in range(0,weiner_len):
            mgf_mean_vec[Index(i)] = weiner[Index(i)]*exp(theta*weiner[Index(i)])
            mgf_var_vec[Index(i)] = (weiner[Index(i)]*weiner[Index(i)])*exp(theta*weiner[Index(i)])
            mean_sum += mgf_mean_vec[Index(i)]
            variance_sum += mgf_var_vec[Index(i)]
            if weiner[Index(i)] > 0:
                pos_count +=1

        var neg_count = weiner_len - pos_count 
        print("Positive Count", pos_count)
        print("Negative Count", neg_count)
        _ = self.variance_1d(weiner)
        return(weiner)

    fn mean_1d(self, weiner: Tensor[DType.float64]) -> SIMD[DType.float64,1]:
        var mu: Float64 = 0
        for i in range(0,weiner.num_elements()):
            mu += weiner[Index(i)]
        return(mu/weiner.num_elements())

    fn variance_1d(self, weiner: Tensor[DType.float64]) -> SIMD[DType.float64,1]:
        var mu = self.mean_1d(weiner)
        var sigma: Float64 = 0
        for i in range(0, weiner.num_elements()):
            sigma += ((weiner[Index(i)]-mu)**2)/weiner.num_elements()
        print("Mean", mu)
        print("Sigma", sqrt(sigma))
        return(sigma)

struct GeomBM:
    var num_paths: Int
    var time_steps: Int
    var time_span: Int
    var power: Int
    var mu: Float64
    var sigma: Float64
    var sim_paths: TensorDict
    var s_0: Float64

    fn __init__(out self, 
                num_paths: Int, 
                time_span: Int,
                power: Int, 
                mu: Float64, 
                sigma: Float64,
                sim_paths: TensorDict,
                s_0: Float64):
        self.num_paths = Int(num_paths)
        self.time_span = time_span
        self.power = power
        self.mu = mu
        self.sigma = sigma
        self.time_steps = 2**power
        self.sim_paths = sim_paths
        self.s_0 = s_0

    def gen_weiner_paths(mut self):
        var brown_brdg = BrownianBridge(24,1, TensorDict())
        var t_vec = gen_time_vec(self.time_steps)
        self.sim_paths.set("Time", t_vec)
        for i in range(self.num_paths):
            self.sim_paths.set(String(i), brown_brdg.generate_bridge())
            print("Generated Path", i)

    def gen_mcs_lazy(mut self) -> Tensor[DType.float64]:
        _ = self.gen_weiner_paths()
        var t_vec = gen_time_vec(self.time_steps)
        var simulation = Tensor[DType.float64](TensorSpec(DType.float64,TensorShape(self.num_paths, self.time_steps)))
        var tmp: Tensor[DType.float64]
        for i in range(0,self.num_paths-1):
            tmp = self.sim_paths.get[DType.float64](String(i))
            for j in range(1,self.time_steps):
                simulation[Index(i,j)] = self.s_0*exp((self.mu - 0.5*self.sigma**2)*t_vec[Index(j,0)]-
                                                      t_vec[Index(j-1,0)]+self.sigma*(tmp[Index(j)]-tmp[Index(j-1)]))
                #print(simulation[Index(i,j)])
        return(simulation)


                




    #fn geom_bm_dict(self) -> TensorDict:
    #    var tensors = TensorDict()
    #    var brown_brdg = BrownianBridge(self.power, self.time_span)
    #    var weiner: Tensor[DType.float64]
    #    var geom_bm_tensor = Tensor[DType.float64](DType.float64,TensorShape(self.num_paths,self.time_steps)
    #    for i in range(num_paths):
    #        weiner = brown_brdg.generate_bridge()
    #        for j in range(0,weiner.num_elements()):
