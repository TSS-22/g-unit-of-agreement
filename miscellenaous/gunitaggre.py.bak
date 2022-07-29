def g_unit_aggr_2p(arg_x, arg_y, arg_values,arg_precision=100, arg_stdDev=1, arg_widthDistri=3.5):  
    # Function to compute the density function of the standard normal distribution
    def standard_norm_dist(arg_stdDev, arg_precision):
        x = np.linspace(-arg_stdDev,arg_stdDev,arg_precision)
        stdDistri = np.exp(-0.5*(x)**2)/np.sqrt(2*np.pi)
        return prob_density

    def range_values(x, FACTOR_STD, arg_precision):
        return np.linspace(x-FACTOR_STD,x+FACTOR_STD,arg_precision)

    def area2distri(val1,val2,stdDistri):
        return np.sum(np.fmin(stdDistri[val1<=np.max(val2)] ,stdDistri[val2>=np.min(val1)]))/np.sum(stdDistri)

    std_range = np.std(arg_values)*arg_stdDev

    stdDistri = standard_norm_dist(arg_widthDistri, arg_precision)

    x_range = range_values(arg_x, std_range, arg_precision)
    y_range = range_values(arg_y, std_range, arg_precision)   

    print(stdDistri)

    if arg_x <= arg_y:
        gUnitAggr = area2distri(y_range,x_range,stdDistri)
    else:
        gUnitAggr = area2distri(x_range,y_range,stdDistri)

    return gUnitAggr