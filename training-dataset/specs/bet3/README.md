# bet3 preamble is alp14's, deliberately

2.0 BET3 has no gold specs and no preamble of its own. This is alp14's, copied.

It is NOT adequate vocabulary for this document and is not meant to be: 71 of the
186 Rmi/Rsi/Psci types the bet3 sections use are absent from it, including
`RmiResult`, which appears 104 times and is what alp14 called
`RmiCommandReturnCode`. Nearly every generated spec will therefore fail to
compile, so compile rate and the Z3 unsat/vacuous sweep are NOT interpretable on
bet3 and are not reported.

It is here because the dangling-output check reads the generated signature and
body and never invokes Verus, so that measurement survives the vocabulary drift.
The preamble still serves its other purpose: showing the model the house style
and the helper names that did carry over.
