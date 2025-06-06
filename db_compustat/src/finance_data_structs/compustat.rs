use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CompQtrly {
    pub gvkey: i32,
    pub datadate: NaiveDate,
    pub fyearq: Option<i32>,
    pub fqtr: Option<i32>,
    pub fyr: Option<i32>,
    pub indfmt: Cow<'static, str>,
    pub consol: Cow<'static, str>,
    pub popsrc: Cow<'static, str>,
    pub datafmt: Cow<'static, str>,
    pub tic: Cow<'static, str>,
    pub cusip: Cow<'static, str>,
    pub conm: Cow<'static, str>,
    pub curcdq: Cow<'static, str>,
    pub datacqtr: Cow<'static, str>,
    pub datafqtr: Cow<'static, str>,
    pub acchgq: Option<f64>,
    pub acomincq: Option<f64>,
    pub acoq: Option<f64>,
    pub actq: Option<f64>,
    pub altoq: Option<f64>,
    pub ancq: Option<f64>,
    pub anoq: Option<f64>,
    pub aociderglq: Option<f64>,
    pub aociotherq: Option<f64>,
    pub aocipenq: Option<f64>,
    pub aocisecglq: Option<f64>,
    pub aol2q: Option<f64>,
    pub aoq: Option<f64>,
    pub apq: Option<f64>,
    pub aqaq: Option<f64>,
    pub aqdq: Option<f64>,
    pub aqepsq: Option<f64>,
    pub aqpl1q: Option<f64>,
    pub aqpq: Option<f64>,
    pub arcedq: Option<f64>,
    pub arceepsq: Option<f64>,
    pub arceq: Option<f64>,
    pub atq: Option<f64>,
    pub aul3q: Option<f64>,
    pub billexceq: Option<f64>,
    pub capr1q: Option<f64>,
    pub capr2q: Option<f64>,
    pub capr3q: Option<f64>,
    pub capsftq: Option<f64>,
    pub capsq: Option<f64>,
    pub ceiexbillq: Option<f64>,
    pub ceqq: Option<f64>,
    pub cheq: Option<f64>,
    pub chq: Option<f64>,
    pub cibegniq: Option<f64>,
    pub cicurrq: Option<f64>,
    pub ciderglq: Option<f64>,
    pub cimiiq: Option<f64>,
    pub ciotherq: Option<f64>,
    pub cipenq: Option<f64>,
    pub ciq: Option<f64>,
    pub cisecglq: Option<f64>,
    pub citotalq: Option<f64>,
    pub cogsq: Option<f64>,
    pub csh12q: Option<f64>,
    pub cshfd12: Option<f64>,
    pub cshfdq: Option<f64>,
    pub cshiq: Option<f64>,
    pub cshopq: Option<f64>,
    pub cshoq: Option<f64>,
    pub cshprq: Option<f64>,
    pub cstkcvq: Option<f64>,
    pub cstkeq: Option<f64>,
    pub cstkq: Option<f64>,
    pub dcomq: Option<f64>,
    pub dd1q: Option<f64>,
    pub deracq: Option<f64>,
    pub deraltq: Option<f64>,
    pub derhedglq: Option<f64>,
    pub derlcq: Option<f64>,
    pub derlltq: Option<f64>,
    pub diladq: Option<f64>,
    pub dilavq: Option<f64>,
    pub dlcq: Option<f64>,
    pub dlttq: Option<f64>,
    pub doq: Option<f64>,
    pub dpacreq: Option<f64>,
    pub dpactq: Option<f64>,
    pub dpq: Option<f64>,
    pub dpretq: Option<f64>,
    pub drcq: Option<f64>,
    pub drltq: Option<f64>,
    pub dteaq: Option<f64>,
    pub dtedq: Option<f64>,
    pub dteepsq: Option<f64>,
    pub dtepq: Option<f64>,
    pub dvintfq: Option<f64>,
    pub dvpq: Option<f64>,
    pub epsf12: Option<f64>,
    pub epsfi12: Option<f64>,
    pub epsfiq: Option<f64>,
    pub epsfxq: Option<f64>,
    pub epspi12: Option<f64>,
    pub epspiq: Option<f64>,
    pub epspxq: Option<f64>,
    pub epsx12: Option<f64>,
    pub esopctq: Option<f64>,
    pub esopnrq: Option<f64>,
    pub esoprq: Option<f64>,
    pub esoptq: Option<f64>,
    pub esubq: Option<f64>,
    pub fcaq: Option<f64>,
    pub ffoq: Option<f64>,
    pub finacoq: Option<f64>,
    pub finaoq: Option<f64>,
    pub finchq: Option<f64>,
    pub findlcq: Option<f64>,
    pub findltq: Option<f64>,
    pub finivstq: Option<f64>,
    pub finlcoq: Option<f64>,
    pub finltoq: Option<f64>,
    pub finnpq: Option<f64>,
    pub finreccq: Option<f64>,
    pub finrecltq: Option<f64>,
    pub finrevq: Option<f64>,
    pub finxintq: Option<f64>,
    pub finxoprq: Option<f64>,
    pub gdwlamq: Option<f64>,
    pub gdwlia12: Option<f64>,
    pub gdwliaq: Option<f64>,
    pub gdwlid12: Option<f64>,
    pub gdwlidq: Option<f64>,
    pub gdwlieps12: Option<f64>,
    pub gdwliepsq: Option<f64>,
    pub gdwlipq: Option<f64>,
    pub gdwlq: Option<f64>,
    pub glaq: Option<f64>,
    pub glcea12: Option<f64>,
    pub glceaq: Option<f64>,
    pub glced12: Option<f64>,
    pub glcedq: Option<f64>,
    pub glceeps12: Option<f64>,
    pub glceepsq: Option<f64>,
    pub glcepq: Option<f64>,
    pub gldq: Option<f64>,
    pub glepsq: Option<f64>,
    pub glivq: Option<f64>,
    pub glpq: Option<f64>,
    pub hedgeglq: Option<f64>,
    pub ibadj12: Option<f64>,
    pub ibadjq: Option<f64>,
    pub ibcomq: Option<f64>,
    pub ibmiiq: Option<f64>,
    pub ibq: Option<f64>,
    pub icaptq: Option<f64>,
    pub intaccq: Option<f64>,
    pub intanoq: Option<f64>,
    pub intanq: Option<f64>,
    pub invfgq: Option<f64>,
    pub invoq: Option<f64>,
    pub invrmq: Option<f64>,
    pub invtq: Option<f64>,
    pub invwipq: Option<f64>,
    pub ivaeqq: Option<f64>,
    pub ivaoq: Option<f64>,
    pub ivltq: Option<f64>,
    pub ivstq: Option<f64>,
    pub lcoq: Option<f64>,
    pub lctq: Option<f64>,
    pub lltq: Option<f64>,
    pub lnoq: Option<f64>,
    pub lol2q: Option<f64>,
    pub loq: Option<f64>,
    pub loxdrq: Option<f64>,
    pub lqpl1q: Option<f64>,
    pub lseq: Option<f64>,
    pub ltmibq: Option<f64>,
    pub ltq: Option<f64>,
    pub lul3q: Option<f64>,
    pub mibnq: Option<f64>,
    pub mibq: Option<f64>,
    pub mibtq: Option<f64>,
    pub miiq: Option<f64>,
    pub msaq: Option<f64>,
    pub ncoq: Option<f64>,
    pub niitq: Option<f64>,
    pub nimq: Option<f64>,
    pub niq: Option<f64>,
    pub nopiq: Option<f64>,
    pub npatq: Option<f64>,
    pub npq: Option<f64>,
    pub nrtxtdq: Option<f64>,
    pub nrtxtepsq: Option<f64>,
    pub nrtxtq: Option<f64>,
    pub obkq: Option<f64>,
    pub oepf12: Option<f64>,
    pub oeps12: Option<f64>,
    pub oepsxq: Option<f64>,
    pub oiadpq: Option<f64>,
    pub oibdpq: Option<f64>,
    pub opepsq: Option<f64>,
    pub optdrq: Option<f64>,
    pub optfvgrq: Option<f64>,
    pub optlifeq: Option<f64>,
    pub optrfrq: Option<f64>,
    pub optvolq: Option<f64>,
    pub piq: Option<f64>,
    pub pllq: Option<f64>,
    pub pnc12: Option<f64>,
    pub pncd12: Option<f64>,
    pub pncdq: Option<f64>,
    pub pnceps12: Option<f64>,
    pub pncepsq: Option<f64>,
    pub pnciapq: Option<f64>,
    pub pnciaq: Option<f64>,
    pub pncidpq: Option<f64>,
    pub pncidq: Option<f64>,
    pub pnciepspq: Option<f64>,
    pub pnciepsq: Option<f64>,
    pub pncippq: Option<f64>,
    pub pncipq: Option<f64>,
    pub pncpd12: Option<f64>,
    pub pncpdq: Option<f64>,
    pub pncpeps12: Option<f64>,
    pub pncpepsq: Option<f64>,
    pub pncpq: Option<f64>,
    pub pncq: Option<f64>,
    pub pncwiapq: Option<f64>,
    pub pncwiaq: Option<f64>,
    pub pncwidpq: Option<f64>,
    pub pncwidq: Option<f64>,
    pub pncwiepq: Option<f64>,
    pub pncwiepsq: Option<f64>,
    pub pncwippq: Option<f64>,
    pub pncwipq: Option<f64>,
    pub pnrshoq: Option<f64>,
    pub ppegtq: Option<f64>,
    pub ppentq: Option<f64>,
    pub prcaq: Option<f64>,
    pub prcd12: Option<f64>,
    pub prcdq: Option<f64>,
    pub prce12: Option<f64>,
    pub prceps12: Option<f64>,
    pub prcepsq: Option<f64>,
    pub prcpd12: Option<f64>,
    pub prcpdq: Option<f64>,
    pub prcpeps12: Option<f64>,
    pub prcpepsq: Option<f64>,
    pub prcpq: Option<f64>,
    pub prcraq: Option<f64>,
    pub prshoq: Option<f64>,
    pub pstknq: Option<f64>,
    pub pstkq: Option<f64>,
    pub pstkrq: Option<f64>,
    pub rcaq: Option<f64>,
    pub rcdq: Option<f64>,
    pub rcepsq: Option<f64>,
    pub rcpq: Option<f64>,
    pub rdipaq: Option<f64>,
    pub rdipdq: Option<f64>,
    pub rdipepsq: Option<f64>,
    pub rdipq: Option<f64>,
    pub recdq: Option<f64>,
    pub rectaq: Option<f64>,
    pub rectoq: Option<f64>,
    pub rectq: Option<f64>,
    pub rectrq: Option<f64>,
    pub recubq: Option<f64>,
    pub req: Option<f64>,
    pub retq: Option<f64>,
    pub reunaq: Option<f64>,
    pub revtq: Option<f64>,
    pub rllq: Option<f64>,
    pub rra12: Option<f64>,
    pub rraq: Option<f64>,
    pub rrd12: Option<f64>,
    pub rrdq: Option<f64>,
    pub rreps12: Option<f64>,
    pub rrepsq: Option<f64>,
    pub rrpq: Option<f64>,
    pub rstcheltq: Option<f64>,
    pub rstcheq: Option<f64>,
    pub saleq: Option<f64>,
    pub seqoq: Option<f64>,
    pub seqq: Option<f64>,
    pub seta12: Option<f64>,
    pub setaq: Option<f64>,
    pub setd12: Option<f64>,
    pub setdq: Option<f64>,
    pub seteps12: Option<f64>,
    pub setepsq: Option<f64>,
    pub setpq: Option<f64>,
    pub spce12: Option<f64>,
    pub spced12: Option<f64>,
    pub spcedpq: Option<f64>,
    pub spcedq: Option<f64>,
    pub spceeps12: Option<f64>,
    pub spceepsp12: Option<f64>,
    pub spceepspq: Option<f64>,
    pub spceepsq: Option<f64>,
    pub spcep12: Option<f64>,
    pub spcepd12: Option<f64>,
    pub spcepq: Option<f64>,
    pub spceq: Option<f64>,
    pub spidq: Option<f64>,
    pub spiepsq: Option<f64>,
    pub spioaq: Option<f64>,
    pub spiopq: Option<f64>,
    pub spiq: Option<f64>,
    pub sretq: Option<f64>,
    pub stkcoq: Option<f64>,
    pub stkcpaq: Option<f64>,
    pub teqq: Option<f64>,
    pub tfvaq: Option<f64>,
    pub tfvceq: Option<f64>,
    pub tfvlq: Option<f64>,
    pub tieq: Option<f64>,
    pub tiiq: Option<f64>,
    pub tstknq: Option<f64>,
    pub tstkq: Option<f64>,
    pub txdbaq: Option<f64>,
    pub txdbcaq: Option<f64>,
    pub txdbclq: Option<f64>,
    pub txdbq: Option<f64>,
    pub txdiq: Option<f64>,
    pub txditcq: Option<f64>,
    pub txpq: Option<f64>,
    pub txtq: Option<f64>,
    pub txwq: Option<f64>,
    pub uacoq: Option<f64>,
    pub uaoq: Option<f64>,
    pub uaptq: Option<f64>,
    pub ucapsq: Option<f64>,
    pub ucconsq: Option<f64>,
    pub uceqq: Option<f64>,
    pub uddq: Option<f64>,
    pub udmbq: Option<f64>,
    pub udoltq: Option<f64>,
    pub udpcoq: Option<f64>,
    pub udvpq: Option<f64>,
    pub ugiq: Option<f64>,
    pub uinvq: Option<f64>,
    pub ulcoq: Option<f64>,
    pub uniamiq: Option<f64>,
    pub unopincq: Option<f64>,
    pub uopiq: Option<f64>,
    pub updvpq: Option<f64>,
    pub upmcstkq: Option<f64>,
    pub upmpfq: Option<f64>,
    pub upmpfsq: Option<f64>,
    pub upmsubpq: Option<f64>,
    pub upstkcq: Option<f64>,
    pub upstkq: Option<f64>,
    pub urectq: Option<f64>,
    pub uspiq: Option<f64>,
    pub usubdvpq: Option<f64>,
    pub usubpcvq: Option<f64>,
    pub utemq: Option<f64>,
    pub wcapq: Option<f64>,
    pub wdaq: Option<f64>,
    pub wddq: Option<f64>,
    pub wdepsq: Option<f64>,
    pub wdpq: Option<f64>,
    pub xaccq: Option<f64>,
    pub xidoq: Option<f64>,
    pub xintq: Option<f64>,
    pub xiq: Option<f64>,
    pub xoprq: Option<f64>,
    pub xopt12: Option<f64>,
    pub xoptd12: Option<f64>,
    pub xoptd12p: Option<f64>,
    pub xoptdq: Option<f64>,
    pub xoptdqp: Option<f64>,
    pub xopteps12: Option<f64>,
    pub xoptepsp12: Option<f64>,
    pub xoptepsq: Option<f64>,
    pub xoptepsqp: Option<f64>,
    pub xoptq: Option<f64>,
    pub xoptqp: Option<f64>,
    pub xrdq: Option<f64>, // Research and development expenses (optional due to NA values)
    pub xsgaq: Option<f64>, // Selling, General & Admin expenses (SG&A)
    pub exchg: Option<i32>, // Exchange code
    pub cik: Option<i32>,  // Central Index Key (CIK)
    pub costat: Cow<'static, str>, // Company status (A for Active, I for Inactive)
    pub fic: Cow<'static, str>, // Country code (FIC)
    pub conml: Cow<'static, str>, // Company name
    pub county: Cow<'static, str>, // County (empty Cow<'static, str> in the dataset)
}
