Many modern algorithms of data encoding use bipartite graphs with great girth for error correction during data transmission over noise channels.
Common approaches to construction of such graphs include methods based on random permutation of matrices, structured methods based on predetermined structures as well as various other methods. 
Many of these methods can not reliably produce bipartite graphs with girth greater than 12.

We are proposing an algorithm that allows the constrcuction of bipartite graphs with any given girth.
We use special subset of graphs called metagraphs to produce systems of inequalities.
The solution to these systems then can be applied to metagraph to construct a new graph with greater girth.
The constructed graph can be later used as a metagraph in the same process.

It was proven that any system of inequalities produced in this recursive process has a solution.
And that using such an algorithm we can construct (m,n)-regular bipartite graph with any given girth. 