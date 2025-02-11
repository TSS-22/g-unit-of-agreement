import numpy as np

# Shanon entropy, +0.1 is addedd to prevent the error of divided by zero

def entropy_info(arg_matrix):
    return np.sum(np.power(arg_matrix,2)*np.log(np.power(arg_matrix,2)+0.1))

def area2distri(val1,val2,width_distri):
    if (val1-val2)/width_distri >=1:
        return 0
    else:
        return (1-(val1-val2)/width_distri)**1*np.exp(1)/np.exp((1-(val1-val2)/width_distri)**1)

def g_unit_aggr(data, width_distri=1):  
    std_range = np.std(data)*width_distri
    
    # Create a vector in order to store the values of commonality between each variables
    temp_aggr=np.zeros(data.shape[1],float)

    # Init matrix to store result
    G_matrix = np.zeros([data.shape[1],data.shape[1]], float)

    # Init counter
    yIdxRow = 0 
    yIdxCol = 0 
    xIdxRow = 0
    
    # Compute the upper triangle (as the matrix is symmetrical) of the g unit values of the matrix
    for val1_row in data:
        # Only take the upper triangle
        for val2_row in data[xIdxRow:,:]:
            for val1, val2 in zip(val1_row, val2_row):
                if val1 <= val2:
                    temp_aggr[yIdxCol] = area2distri(val2,val1,std_range)
                else:
                    temp_aggr[yIdxCol] = area2distri(val1,val2,std_range)
                yIdxCol = yIdxCol+1
            # Store the value in the final matrix, while correcting for the offset due to calculating only the upper triangle
            G_matrix[xIdxRow,xIdxRow+yIdxRow] = np.mean(temp_aggr)
            yIdxRow = yIdxRow+1
            yIdxCol = 0          
        yIdxRow = 0
        xIdxRow=xIdxRow+1

    # We copy back the upper triangle to the lower triangle
    # From https://stackoverflow.com/a/58806735
    G_matrix = G_matrix + G_matrix.T - np.diag(np.diag(G_matrix))

    return G_matrix


def g_unit_aggr_auto(arg_values,arg_resolution=100, arg_stdDev=1, arg_widthDistri=3.5, arg_learning_rate=0.05):
    
    ntrp_g_matrix_n = entropy_info(g_unit_aggr(arg_values,arg_resolution, arg_stdDev, arg_widthDistri))
    ntrp_g_matrix_n1 = entropy_info(g_unit_aggr(arg_values, arg_resolution, arg_stdDev+arg_learning_rate, arg_widthDistri))

    if ntrp_g_matrix_n.abs()>ntrp_g_matrix_n1.abs():
        arg_stdDev=arg_stdDev-arg_learning_rate
        ntrp_g_matrix_n1 = entropy_info(g_unit_aggr(arg_values, arg_resolution, arg_stdDev, arg_widthDistri))
        if ntrp_g_matrix_n.abs()>ntrp_g_matrix_n1.abs():
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
            if ntrp_g_matrix_n.abs()>=ntrp_g_matrix_n1.abs():
                return g_unit_aggr(arg_values, arg_resolution, arg_stdDev-arg_learning_rate, arg_widthDistri)


# DEPRECATED

# def standard_norm_dist(arg_widthDistri, arg_resolution):
#     x = np.linspace(arg_widthDistri,
#         arg_widthDistri,
#         arg_resolution)
#     prob_density = np.exp(-0.5*(x)**2)/np.sqrt(2*np.pi)
#     return prob_density

# def range_values(x, FACTOR_STD, arg_resolution):
#     return np.linspace(x-FACTOR_STD,
#         x+FACTOR_STD,
#         arg_resolution)
        
# def area2distri(val1,val2,stdDistri):
#     return np.sum(np.fmin(stdDistri[val1<=np.max(val2)] ,
#         stdDistri[val2>=np.min(val1)]))/np.sum(stdDistri)

# # arg_values must be a mtrix array, with row = occurences, col = variables
# def g_unit_aggr(arg_values,arg_resolution=100, arg_stdDev=1.0, arg_widthDistri=3.5):  
#     # Create the standard deviation used factor used for aggreement computation
#     std_range = np.std(arg_values)*arg_stdDev

#     # Create the corresponding normal distribution density function
#     stdDistri = standard_norm_dist(arg_widthDistri, arg_resolution)

#     # Create a vector in order to store the values of commonality between each variables
#     temp_aggr=np.zeros(arg_values.shape[1],float)

#     # Init matrix to store result
#     G_matrix = np.zeros([arg_values.shape[1],arg_values.shape[1]], float)
#     range_matrix = np.zeros([arg_values.shape[0],arg_values.shape[1]], np.ndarray)

#     # Create the matrix that store all the range value to only compute them once
#     xIdxRow=0
#     while xIdxRow < arg_values.shape[0]:
#         yIdxCol=0
#         while yIdxCol < arg_values.shape[1]:
#             range_matrix[xIdxRow, yIdxCol] = range_values(arg_values[xIdxRow, yIdxCol], std_range, arg_resolution)
#             yIdxCol=yIdxCol+1
#         xIdxRow=xIdxRow+1

#     # Init counter
#     yIdxRow = 0 
#     yIdxCol = 0 
#     xIdxRow = 0
    
#     # Compute the upper triangle (as the matrix is symmetrical) of the g unit values of the matrix
#     for val1_row in range_matrix:
#         # Only take the upper triangle
#         for val2_row in range_matrix[xIdxRow:,:]:
#             for val1_col, val2_col in zip(val1_row, val2_row):
#                 if np.mean(val1_col) <= np.mean(val2_col):
#                     temp_aggr[yIdxCol] = area2distri(val2_col,val1_col,stdDistri)
#                 else:
#                     temp_aggr[yIdxCol] = area2distri(val1_col,val2_col,stdDistri)
#                 yIdxCol = yIdxCol+1
#             # Store the value in the final matrix, while correcting for the offset due to calculating only the upper triangle
#             G_matrix[xIdxRow,xIdxRow+yIdxRow] = np.mean(temp_aggr)
#             yIdxRow = yIdxRow+1
#             yIdxCol = 0          
#         yIdxRow = 0
#         xIdxRow=xIdxRow+1

#     # We copy back the upper triangle to the lower triangle
#     # From https://stackoverflow.com/a/58806735
#     G_matrix = G_matrix + G_matrix.T - np.diag(np.diag(G_matrix))

#     return G_matrix
