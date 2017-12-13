import scipy.stats as stats
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt


def data_import():
    return pd.read_csv("data_times.csv");

def print_data_demographics(data):
    print("Number of datapoints (rows): " + str(data.index))
    print("Number of datapoints (columns): " + str(data.columns))

def print_parallel_time_1_max(data):
    print("Max value: " + str(max(data.PARALLEL_TIME_1)))

def print_parallel_time_1_min(data):
    print("Max value: " + str(min(data.PARALLEL_TIME_1)))

def print_parallel_time_2_max(data):
    print("Max value: " + str(max(data.PARALLEL_TIME_2)))

def print_parallel_time_2_min(data):
    print("Max value: " + str(min(data.PARALLEL_TIME_2)))

def print_parallel_time_3_max(data):
    print("Max value: " + str(max(data.PARALLEL_TIME_3)))

def print_parallel_time_3_min(data):
    print("Max value: " + str(min(data.PARALLEL_TIME_3)))

def print_parallel_time_4_max(data):
    print("Max value: " + str(max(data.PARALLEL_TIME_4)))

def print_parallel_time_4_min(data):
    print("Max value: " + str(min(data.PARALLEL_TIME_4)))

def print_parallel_time_4_max(data):
    print("Max value: " + str(max(data.PARALLEL_TIME_4)))

def print_parallel_time_4_min(data):
    print("Max value: " + str(min(data.PARALLEL_TIME_4)))

def print_parallel_time_4_max(data):
    print("Max value: " + str(max(data.PARALLEL_TIME_4)))

def print_parallel_time_4_min(data):
    print("Max value: " + str(min(data.PARALLEL_TIME_4)))


##################################################################
##### histograms
##################################################################

def print_parallel_1_hist(data):
    plt.hist(data.PARALLEL_TIME_1,normed=True, bins=5000)
    plt.xlabel("single contained thread [s]")
    plt.ylabel("Frequency")
    #plt.annotate("for maximum", xt =(xpos,ypos))
    plt.show()

def print_parallel_2_hist(data):
    plt.hist(data.PARALLEL_TIME_2,normed=True, bins=5000)
    plt.xlabel("2 parallel thread time [s]")
    plt.ylabel("Frequency")
    #plt.annotate("for maximum", xt =(xpos,ypos))
    plt.show()

def print_parallel_3_hist(data):
    plt.hist(data.PARALLEL_TIME_3,normed=True, bins=5000)
    plt.xlabel("3 parallel thread time [s]")
    plt.ylabel("Frequency")
    #plt.annotate("for maximum", xt =(xpos,ypos))
    plt.show()

def print_parallel_4_hist(data):
    plt.hist(data.PARALLEL_TIME_4,normed=True, bins=5000)
    plt.xlabel("4 parallel thread time [s]")
    plt.ylabel("Frequency")
    #plt.annotate("for maximum", xt =(xpos,ypos))
    plt.show()


##################################################################
##### scatter plots
##################################################################



def print_parallel_time_1_scatter(data):
    colors = 'b'

    plt.scatter(data.PARALLEL_TIME_1, data.TOTAL_TIME)
    plt.title("single contained thread")
    plt.show()

def print_parallel_time_2_scatter(data):
    colors = 'b'

    plt.scatter(data.PARALLEL_TIME_2, data.TOTAL_TIME)
    plt.title("2 threads")
    plt.show()

def print_parallel_time_3_scatter(data):
    colors = 'b'

    plt.scatter(data.PARALLEL_TIME_3, data.TOTAL_TIME)
    plt.title("3 threads")
    plt.show()

def print_parallel_time_4_scatter(data):
    colors = 'b'

    plt.scatter(data.PARALLEL_TIME_4, data.TOTAL_TIME)
    plt.title("4 threads")
    plt.show()



def print_sequential_encode_time_scatter(data):
    colors = 'r'

    plt.scatter(data.SEQUENTIAL_ENCODE, data.TOTAL_TIME)
    plt.title("sequential_encode")
    plt.show()






def main():
    data = data_import()
    #print_data_demographics(data)

    print("single contained thread")
    print_parallel_time_1_max(data)
    print_parallel_time_1_min(data)


    print("2 parallel threads")
    print_parallel_time_2_max(data)
    print_parallel_time_2_min(data)

    print("3 parallel threads")
    print_parallel_time_3_max(data)
    print_parallel_time_3_min(data)

    print("4 parallel threads")
    print_parallel_time_4_max(data)
    print_parallel_time_4_min(data)

    #print_parallel_1_hist(data)
    #print_parallel_2_hist(data)
    #print_parallel_3_hist(data)
    #print_parallel_4_hist(data)


    #print_parallel_time_1_bar(data)
    print_sequential_encode_time_scatter(data)
    print_parallel_time_1_scatter(data)
    print_parallel_time_2_scatter(data)
    print_parallel_time_3_scatter(data)
    print_parallel_time_4_scatter(data)

main()
