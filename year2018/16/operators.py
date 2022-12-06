import operator

def reg_op(r, ins, op):        
    a, b, c = ins
    ret = [None]*len(r)
    for i in range(len(r)):
        if i != c:
            ret[i] = r[i]
    
    ret[c] = op(r[a], r[b])
    return tuple(ret)

def imm_op(r, ins, op):
    a, b, c = ins
    ret = [None]*len(r)
    for i in range(len(r)):
        if i != c:
            ret[i] = r[i]
    
    ret[c] = op(r[a], b)
    return tuple(ret)
        
def addr(r, ins):
    return reg_op(r, ins, operator.add)
    
def addi(r, ins):   
    return imm_op(r, ins, operator.add)
    
def mulr(r, ins):
    return reg_op(r, ins, operator.mul)
    
def muli(r, ins):
    return imm_op(r, ins, operator.mul)
    
def banr(r, ins):
    return reg_op(r, ins, operator.and_)
    
def bani(r, ins):
    return imm_op(r, ins, operator.and_)    

def borr(r, ins):
    return reg_op(r, ins, operator.or_)
    
def bori(r, ins):
    return imm_op(r, ins, operator.or_)
    
def setr(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = r[a]
    return tuple(ret)

def seti(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = a
    return tuple(ret)
    
def gtir(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = int(a > r[b])
    return tuple(ret)

def gtri(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = int(r[a] > b)
    return tuple(ret)
    
def gtrr(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = int(r[a] > r[b])
    return tuple(ret)
    
def eqir(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = int(a == r[b])
    return tuple(ret)

def eqri(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = int(r[a] == b)
    return tuple(ret)
    
def eqrr(r, ins):
    ret = list(r)
    a, b, c = ins
    ret[c] = int(r[a] == r[b])
    return tuple(ret)
    
operators = [
    addr, addi,
    mulr, muli,
    banr, bani,
    borr, bori,
    setr, seti,
    gtir, gtri, gtrr,
    eqir, eqri, eqrr
]