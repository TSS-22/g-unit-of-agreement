import numpy as np
import time

# Shanon entropy, +0.1 is addedd to prevent the error of divided by zero
def entropy_info(arg_matrix):
    return np.sum(np.power(arg_matrix,2)*np.log(np.power(arg_matrix,2)+0.1))

def standard_norm_dist(arg_widthDistri, arg_resolution):
    x = np.linspace(arg_widthDistri,
        arg_widthDistri,
        arg_resolution)
    prob_density = np.exp(-0.5*(x)**2)/np.sqrt(2*np.pi)
    return prob_density

def range_values(x, FACTOR_STD, arg_resolution):
    return np.linspace(x-FACTOR_STD,
        x+FACTOR_STD,
        arg_resolution)
    
def area2distri(val1,val2,stdDistri):
    return np.sum(np.fmin(stdDistri[val1<=np.max(val2)] ,
        stdDistri[val2>=np.min(val1)]))/np.sum(stdDistri)

# arg_values must be a mtrix array, with row = occurences, col = variables
def g_unit_aggr(arg_values,arg_resolution=100, arg_stdDev=1.0, arg_widthDistri=3.5):  
    # Create the standard deviation used factor used for aggreement computation
    std_range = np.std(arg_values)*arg_stdDev

    # Create the corresponding normal distribution density function
    stdDistri = standard_norm_dist(arg_widthDistri, arg_resolution)

    temp_aggr=np.zeros(arg_values.shape[1],float)

    # Init matrix to store result
    G_matrix = np.zeros([arg_values.shape[1],arg_values.shape[1]], float)
    range_matrix = np.zeros([arg_values.shape[0],arg_values.shape[1]], np.ndarray)

    xIdxRow=0
    while xIdxRow < arg_values.shape[0]:
        yIdxCol=0
        while yIdxCol < arg_values.shape[1]:
            range_matrix[xIdxRow, yIdxCol] = range_values(arg_values[xIdxRow, yIdxCol], std_range, arg_resolution)
            yIdxCol=yIdxCol+1
        xIdxRow=xIdxRow+1

    # Init counter
    xIdxRow = 0
    yIdxRow = 0 
    xIdxCol = 0
    yIdxCol = 0 

    
    while xIdxRow < arg_values.shape[0]:
        while xIdxCol < arg_values.shape[1]:
            while yIdxRow < arg_values.shape[0]:
                while yIdxCol < arg_values.shape[1]:
                    if arg_values[xIdxRow,yIdxCol] <= arg_values[yIdxRow,yIdxCol]:
                        temp_aggr[yIdxCol] = area2distri(range_matrix[yIdxRow,yIdxCol],range_matrix[xIdxRow,yIdxCol],stdDistri)
                    else:
                        temp_aggr[yIdxCol] = area2distri(range_matrix[xIdxRow,yIdxCol],range_matrix[yIdxRow,yIdxCol],stdDistri)
                    yIdxCol = yIdxCol+1

                G_matrix[xIdxRow,yIdxRow] = np.mean(temp_aggr)
                yIdxCol = 0 
                yIdxRow = yIdxRow+1
            
            yIdxRow = 0
            xIdxCol=xIdxCol+1
        
        xIdxCol = 0
        xIdxRow=xIdxRow+1
    return G_matrix

def g_unit_aggr_auto(arg_values,arg_resolution=100, arg_stdDev=1, arg_widthDistri=3.5, arg_learning_rate=0.05):
    
    ntrp_g_matrix_n = entropy_info(g_unit_aggr(arg_values,arg_resolution, arg_stdDev, arg_widthDistri))
    ntrp_g_matrix_n1 = entropy_info(g_unit_aggr(arg_values, arg_resolution, arg_stdDev+arg_learning_rate, arg_widthDistri))

    if ntrp_g_matrix_n>ntrp_g_matrix_n1:
        arg_stdDev=arg_stdDev-arg_learning_rate
        ntrp_g_matrix_n1 = entropy_info(g_unit_aggr(arg_values, arg_resolution, arg_stdDev, arg_widthDistri))
        if ntrp_g_matrix_n>ntrp_g_matrix_n1:
            return g_unit_aggr(arg_values, arg_resolution, arg_stdDev, arg_widthDistri)
        else:
            while True:
                arg_stdDev=arg_stdDev-arg_learning_rate
                ntrp_g_matrix_n1 = entropy_info(g_unit_aggr(arg_values, arg_resolution, arg_stdDev, arg_widthDistri))
                print(''+str(ntrp_g_matrix_n)+''+str(ntrp_g_matrix_n1))
                if ntrp_g_matrix_n>=ntrp_g_matrix_n1:
                    return g_unit_aggr(arg_values, arg_resolution, arg_stdDev+arg_learning_rate, arg_widthDistri)
    else:
        while True:
            arg_stdDev=arg_stdDev+arg_learning_rate
            ntrp_g_matrix_n1 = entropy_info(g_unit_aggr(arg_values, arg_resolution, arg_widthDistri, arg_stdDev))
            if ntrp_g_matrix_n>=ntrp_g_matrix_n1:
                return g_unit_aggr(arg_values, arg_resolution, arg_stdDev-arg_learning_rate, arg_widthDistri)




# DEBUG FUNCTION TO GET THE RANGE MATRIX
def range_matrix(arg_values,arg_resolution=100, arg_stdDev=1.0, arg_widthDistri=3.5):  
    # Create the standard deviation used factor used for aggreement computation
    std_range = np.std(arg_values)*arg_stdDev

    range_matrix = np.zeros([arg_values.shape[0],arg_values.shape[1]], np.ndarray)
    xIdxRow=0
    while xIdxRow < arg_values.shape[0]:
        yIdxCol=0
        while yIdxCol < arg_values.shape[1]:
            range_matrix[xIdxRow, yIdxCol] = range_values(arg_values[xIdxRow, yIdxCol], std_range, arg_resolution)
            yIdxCol=yIdxCol+1
        xIdxRow=xIdxRow+1

    return range_matrix
