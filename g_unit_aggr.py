import numpy as np

def standard_norm_dist(arg_stdDev, arg_precision):
    x = np.linspace(-arg_stdDev,arg_stdDev,arg_precision)
    prob_density = np.exp(-0.5*(x)**2)/np.sqrt(2*np.pi)
    return prob_density

def range_values(x, FACTOR_STD, arg_precision):
    return np.linspace(x-FACTOR_STD,x+FACTOR_STD,arg_precision)
    
def area2distri(val1,val2,stdDistri):
    return np.sum(np.fmin(stdDistri[val1<=np.max(val2)] ,stdDistri[val2>=np.min(val1)]))/np.sum(stdDistri)

# arg_values must be a mtrix array, with row = occurences, col = variables
def g_unit_aggr(arg_values,arg_precision=100, arg_stdDev=1, arg_widthDistri=3.5):  
    # Create the standard deviation used factor used for aggreement computation
    std_range = np.std(arg_values)*arg_stdDev

    # Create the corresponding normal distribution density function
    stdDistri = standard_norm_dist(arg_widthDistri, arg_precision)

    # Init counter
    xIdxRow = 0
    yIdxRow = 0 
    xIdxCol = 0
    yIdxCol = 0 

    temp_aggr=np.zeros(arg_values.shape[1],float)

    # Init matrix to store result
    G_matrix = np.zeros([arg_values.shape[1],arg_values.shape[1]], float)
    #range_matrix = np.zeros([arg_values.shape[0],arg_values.shape[1]], float)

    while xIdxRow < arg_values.shape[0]:
        while xIdxCol < arg_values.shape[1]:
            while yIdxRow < arg_values.shape[0]:
                while yIdxCol < arg_values.shape[1]:
                    x_range = range_values(arg_values[xIdxRow,yIdxCol], std_range, arg_precision)
                    y_range = range_values(arg_values[yIdxRow,yIdxCol], std_range, arg_precision)
                    
                    if arg_values[xIdxRow,xIdxCol] <= arg_values[yIdxRow,yIdxCol]:
                        temp_aggr[yIdxCol] = area2distri(y_range,x_range,stdDistri)
                    else:
                        temp_aggr[yIdxCol] = area2distri(x_range,y_range,stdDistri)
                    yIdxCol = yIdxCol+1

                G_matrix[xIdxRow,yIdxRow] = np.mean(temp_aggr)
                yIdxCol = 0 
                yIdxRow = yIdxRow+1
            
            yIdxRow = 0
            xIdxCol=xIdxCol+1
        
        xIdxCol = 0
        xIdxRow=xIdxRow+1

    return G_matrix


       
        

        