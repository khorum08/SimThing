\# SimThing EML Integration Guidance



\## Purpose



This document defines how SimThing should evaluate and potentially integrate \*\*EML\*\* — the `exp(x) - log(y)` elementary-function construction — into the scripting, derived-field, and Clausewitz-style transpilation layers.



EML is interesting because it offers a uniform mathematical target: many ordinary numeric formulas can be represented as trees of a single binary operator.



For SimThing, the correct use of EML is \*\*not\*\* to replace the existing simulation architecture.



The correct use is:



> EML may become a backend for pure numeric expressions, derived fields, AI weights, and composite predicates. It should not become the primary scripting language, the event system, or the direct replacement for overlays, thresholds, or boundary requests.



\---



\# 1. Summary Position



EML is potentially useful as a \*\*low-level expression backend\*\*.



It is not a full game scripting model.



SimThing already has native mechanisms for the most important simulation concepts:



```text

modifiers       → overlays

state change    → BoundaryRequest

unlock effects  → Suspended overlays + ActivateOverlay

thresholds      → ThresholdRegistration

aggregation     → ReductionRule

AI fields       → output\_vectors / derived fields

runtime truth   → GPU values + CPU semantic interpretation



EML should be introduced only where these native primitives are insufficient or where a uniform numeric expression tree has clear value.



The guiding rule:



Use native SimThing primitives where possible.

Use EML only for pure numeric expressions that need a uniform compiled backend.

2\. What EML Is



The EML paper proposes that a single binary operator:



EML(x, y) = exp(x) - log(y)



together with constants and tree composition, can represent a large class of ordinary elementary functions.



The practical architectural value is not that SimThing would use fewer conceptual operators. The value is that heterogeneous numeric formulas could be lowered into one uniform tree representation.



That gives SimThing a possible backend for:



derived fields

AI weights

risk/opportunity frontiers

numeric scripted values

composite trigger scores

designer-authored formulas

symbolic regression experiments



The cost is that EML introduces nontrivial numerical and implementation concerns:



log-domain safety

exp overflow

NaN / infinity propagation

tree-size growth

debuggability

loss of designer legibility

possible need for clamping and totalized semantics



Therefore EML should be hidden behind a higher-level SimThing Script IR.



3\. Do Not Transpile Clausewitz Directly to EML



Clausewitz-style scripting is not just math.



It includes:



triggers

effects

modifiers

scripted values

scripted triggers

scripted effects

event scopes

random lists

AI weights

event options

localization references

asset references

scope traversal



Only a subset of that is pure numeric expression evaluation.



Therefore the transpilation target should not be:



Clausewitz script → EML



The correct pipeline is:



Clausewitz-like config

&#x20; → parsed AST

&#x20; → typed scoped SimThing Script IR

&#x20; → semantic lowering:

&#x20;       modifiers       → overlays

&#x20;       effects         → BoundaryRequest

&#x20;       triggers        → thresholds / predicates

&#x20;       scripted values → derived expressions

&#x20;       AI weights      → frontier expressions

&#x20; → optional numeric lowering:

&#x20;       pure expressions → native GPU expression graph or EML



EML is a backend for part of the system, not the whole system.



4\. Recommended Architecture

4.1 Canonical Script IR First



Before EML, SimThing should define a canonical expression and predicate IR.



Example sketch:



pub enum ScriptExpr {

&#x20;   Const(f32),



&#x20;   Read {

&#x20;       scope: ScopeRef,

&#x20;       property: PropertyKey,

&#x20;       role: SubFieldRole,

&#x20;   },



&#x20;   Add(Box<ScriptExpr>, Box<ScriptExpr>),

&#x20;   Sub(Box<ScriptExpr>, Box<ScriptExpr>),

&#x20;   Mul(Box<ScriptExpr>, Box<ScriptExpr>),

&#x20;   Div(Box<ScriptExpr>, Box<ScriptExpr>),



&#x20;   Min(Box<ScriptExpr>, Box<ScriptExpr>),

&#x20;   Max(Box<ScriptExpr>, Box<ScriptExpr>),



&#x20;   Clamp {

&#x20;       value: Box<ScriptExpr>,

&#x20;       min: f32,

&#x20;       max: f32,

&#x20;   },



&#x20;   Gate(Box<ScriptPredicate>),

}



And:



pub enum ScriptPredicate {

&#x20;   Greater(ScriptExpr, ScriptExpr),

&#x20;   Less(ScriptExpr, ScriptExpr),

&#x20;   Equalish(ScriptExpr, ScriptExpr),



&#x20;   HasOverlay(OverlayKey),

&#x20;   HasCapability(CapabilityKey),



&#x20;   And(Vec<ScriptPredicate>),

&#x20;   Or(Vec<ScriptPredicate>),

&#x20;   Not(Box<ScriptPredicate>),

}



The Script IR should remain the source of truth.



EML lowering should be one backend:



ScriptExpr

&#x20; → CPU reference evaluator

&#x20; → native GPU evaluator

&#x20; → EML tree backend

&#x20; → explanation tree / designer UI

4.2 EML Backend Is Optional



The compiler should support multiple lowering paths:



Simple Add/Multiply/Set

&#x20; → native overlay or intent delta



Simple threshold comparison

&#x20; → ThresholdRegistration



Simple reduction

&#x20; → ReductionRule



Complex pure numeric expression

&#x20; → native expression graph or EML



Effectful script

&#x20; → BoundaryRequest / event handling



EML should be selected only when:



the expression is pure

all scopes are resolved

all inputs are numeric

the expression has no side effects

domain safety can be guaranteed or guarded

the EML tree size is acceptable

the backend is enabled for that build/profile

5\. Good EML Use Cases

5.1 Derived Fields



Derived fields are the best immediate target.



Examples:



migration\_pull

trade\_pull

rebellion\_pressure

fleet\_projection

disease\_pressure

visibility\_confidence

colonization\_score



These are usually continuous numeric functions over existing SimThing properties.



Example:



rebellion\_pressure =

&#x20;   grievance \* 1.5

&#x20; + food\_insecurity \* 0.8

&#x20; - fleet\_projection \* 0.4

&#x20; + loyalty\_velocity\_abs \* 2.0



This is a pure numeric expression. It can be compiled to a derived field and optionally lowered to EML.



5.2 AI Weights and Risk Frontiers



AI frontier scoring is a strong EML candidate.



Examples:



attack\_opportunity

revolt\_support\_score

colonization\_priority

trade\_route\_desirability

diplomatic\_acceptance\_score



AI should not repeatedly traverse semantic trees to rediscover these numbers. It should read already-computed risk/opportunity fields.



Pipeline:



designer formula

&#x20; → ScriptExpr

&#x20; → GPU derived field

&#x20; → output\_vectors

&#x20; → AI frontier query



EML can serve as the uniform formula backend for complex scoring functions.



5.3 Scripted Values



Clausewitz-style scripted\_value equivalents map well to ScriptExpr.



Example:



scripted\_value = {

&#x20; base = food\_security

&#x20; add = local\_stability

&#x20; multiply = habitability

}



SimThing should parse this into ScriptExpr, not EML directly.



Then the compiler can decide whether to lower to:



native expression graph

EML tree

CPU-only evaluator

constant-folded value

5.4 Composite Trigger Scores



Simple triggers should remain thresholds.



But composite triggers may become numeric scores:



civil\_war\_risk\_score =

&#x20;   low\_loyalty\_pressure

&#x20; + elite\_discontent

&#x20; + military\_disloyalty

&#x20; - legitimacy



Then:



civil\_war\_risk\_score > threshold



can be registered as a threshold over a derived field.



EML may be used to compute the score.



6\. Poor EML Use Cases

6.1 Direct Effects



Do not encode effects as EML.



Examples of non-EML effects:



create faction

spawn fleet

activate overlay

suspend overlay

add child

remove node

reparent node

restore dimension

create event

record delta log entry



These belong to CPU boundary semantics.



They map to:



BoundaryRequest



or to higher-level semantic event handlers.



6.2 Scope Traversal



EML does not solve scope resolution.



Clausewitz-like scopes include:



root

from

prev

owner

capital

any\_owned\_planet

every\_neighbor\_system

random\_owned\_fleet



Scope traversal is a semantic graph/tree problem. It belongs in the parser, resolver, and Script IR binding layer.



EML should only see already-resolved numeric reads.



6.3 Randomness and Event Scheduling



Do not use EML for:



random\_list

mean\_time\_to\_happen

event delay

cooldowns

weighted random effects



Those remain CPU-side systems.



6.4 Designer-Facing Syntax



Designers should never author raw EML.



They should author:



rebellion\_pressure =

&#x20; sigmoid(grievance \* 1.5 + food\_insecurity - fleet\_projection \* 0.4)



or structured RON / Studio config.



The EML tree is an implementation artifact.



7\. Clausewitz Transpilation Strategy

7.1 Parse First, Interpret Later



The importer should preserve the original structure before interpretation.



Clausewitz-like file

&#x20; → raw AST

&#x20; → classified AST

&#x20; → scoped semantic AST

&#x20; → SimThing Script IR



Do not immediately map syntax to EML.



7.2 Classify Script Blocks



Each parsed block should be classified as one of:



modifier

trigger

effect

scripted value

scripted trigger

scripted effect

event

AI weight

localization / metadata

asset reference



Each class maps to a different SimThing concept.



7.3 Mapping Table

Clausewitz-style concept	SimThing target	EML?

Flat modifier	Overlay / PropertyTransformDelta	No

Timed modifier	Transient overlay	No

Locked/unlocked modifier	Suspended overlay + ActivateOverlay	No

Scripted value	ScriptExpr / derived field	Maybe

AI weight	ScriptExpr / frontier field	Maybe

Simple trigger	ThresholdRegistration / predicate	Usually no

Composite trigger	Derived score + threshold	Maybe

Effect	BoundaryRequest / event handler	No

Scope traversal	Scope resolver / IR binding	No

Random list	CPU event system	No

Localization	Studio metadata	No

7.4 Example: Modifier



Clausewitz-style:



planet\_jobs\_produces\_mult = 0.15



SimThing:



PropertyTransformDelta {

&#x20;   property\_id: economy::job\_output,

&#x20;   sub\_field\_deltas: vec!\[

&#x20;       (SubFieldRole::Named("output"), TransformOp::Multiply(1.15)),

&#x20;   ],

}



No EML needed.



7.5 Example: Tech Unlock



Clausewitz-style:



tech\_ion\_drive = {

&#x20; modifier = {

&#x20;   ship\_speed\_mult = 0.30

&#x20; }

}



SimThing:



Capability tree property:

&#x20; tech::propulsion::ion\_drive\_progress



Suspended overlay:

&#x20; transform: military::fleet\_speed Amount Multiply(1.30)

&#x20; lifecycle: Suspended { when\_activated: Permanent }



Unlock threshold:

&#x20; progress >= research\_cost



Boundary:

&#x20; ActivateOverlay { target: tech\_tree\_node, overlay\_id }



No EML needed unless the research progress formula itself is complex.



7.6 Example: AI Weight



Clausewitz-style:



ai\_weight = {

&#x20; base = 10

&#x20; modifier = {

&#x20;   factor = 2

&#x20;   stability > 0.6

&#x20; }

&#x20; modifier = {

&#x20;   factor = 0.5

&#x20;   food\_security < 0.2

&#x20; }

}



SimThing Script IR:



weight =

&#x20; 10

&#x20; \* if stability > 0.6 then 2 else 1

&#x20; \* if food\_security < 0.2 then 0.5 else 1



Lowering options:



small expression → native GPU expression graph

large expression → EML tree backend

debug mode → CPU reference evaluator

8\. EML Safety Rules



EML introduces exp and log, so safety matters.



The EML backend must define totalized game-safe behavior.



8.1 Required Guards



The EML backend must specify:



safe\_log behavior

safe\_exp behavior

NaN handling

infinity handling

overflow handling

underflow handling

domain clamp policy

fallback value policy

diagnostic reporting



Suggested runtime policy:



safe\_log(x):

&#x20; log(max(x, EPSILON))



safe\_exp(x):

&#x20; exp(clamp(x, EXP\_MIN, EXP\_MAX))



NaN result:

&#x20; replace with configured fallback

&#x20; emit diagnostic counter



Infinity:

&#x20; clamp to configured numeric bounds

&#x20; emit diagnostic counter

8.2 Avoid Complex Runtime Semantics Initially



The EML paper may use complex-valued intermediate reasoning for universality.



SimThing should not introduce complex runtime values into the game simulation until there is a compelling reason.



Initial policy:



real-valued EML only

clamped domains

explicit fallback behavior

debug diagnostics

8.3 Tree Size Limits



Compiled EML trees may be larger than the original expression.



Every EML compile should report:



source node count

EML node count

max tree depth

number of exp/log ops

estimated GPU cost



Studio should warn when an expression exceeds thresholds.



Suggested initial limits:



max\_eml\_nodes\_per\_expression: 256

max\_eml\_depth: 32

max\_exp\_log\_ops: 256



These are placeholders and should be benchmarked.



9\. Debuggability Requirements



EML must remain explainable.



Every EML-lowered expression should retain:



original source expression

Script IR tree

EML tree

source-to-EML node mapping where practical

evaluation diagnostics

fallback/clamp counters



Studio should display the designer expression, not the EML tree.



Debug views may expose:



compiled backend: EML

EML node count

tree depth

last value

NaN/clamp count

input contributors



For player-facing explanations, use the Script IR / contribution tree, not raw EML.



10\. Proposed Crate Boundaries



Do not put EML directly into simthing-core at first.



Recommended future layout:



simthing-script

&#x20; Canonical Script IR

&#x20; Clausewitz-like parser interfaces

&#x20; scope resolver interfaces

&#x20; validation



simthing-eml

&#x20; EML tree representation

&#x20; ScriptExpr → EML compiler

&#x20; CPU reference evaluator

&#x20; safety policy

&#x20; diagnostics



simthing-spec

&#x20; RON → runtime compiler (capability trees first)

&#x20; CapabilityTreeSpec / CapabilityTreeBuilder

&#x20; boundary handler + session-init wiring

&#x20; depends on simthing-core + simthing-feeder only



simthing-studio

&#x20; designer UI (deferred; depends on simthing-spec)



simthing-gpu

&#x20; optional EML evaluation backend if/when needed



simthing-core should remain focused on:



SimThing

properties

overlays

registry

threshold definitions

fission templates

11\. Integration Phases

Phase 0 — Documentation Only



Status: this document.



No implementation required.



Phase 1 — Script IR



Implement a canonical ScriptExpr / ScriptPredicate model.



Goals:



parse simple formulas

resolve property/role references

evaluate on CPU for tests

produce explanation trees



No EML yet.



Phase 2 — Native Lowering



Lower simple expressions to native SimThing concepts:



constant Add/Multiply/Set → overlays

simple comparisons → thresholds

simple reductions → ReductionRule



Still no EML required.



Phase 3 — EML Prototype Backend



Create simthing-eml.



Implement:



EML tree type

ScriptExpr → EML compile

CPU EML evaluator

safe\_log / safe\_exp policy

diagnostics

tree-size reporting



Use only in tests and offline tools.



Phase 4 — Derived Field Integration



Allow selected derived fields to use EML backend.



Example:



derived\_field rebellion\_pressure uses EML backend



Compare against CPU reference evaluator.



Phase 5 — GPU EML Evaluation



Only after profiling shows value.



Implement a GPU evaluator for EML trees if:



native expression graph is insufficient

EML tree sizes are acceptable

batch sizes justify GPU evaluation

numeric safety is proven

12\. Recommended Initial Tests

12.1 Expression Equivalence

ScriptExpr CPU evaluator equals EML evaluator

for simple arithmetic formulas

12.2 Domain Safety

log input <= 0 does not produce NaN

exp overflow clamps

NaN fallback increments diagnostic counter

12.3 Clausewitz Mapping

flat modifier maps to overlay, not EML

simple trigger maps to threshold, not EML

scripted value maps to ScriptExpr

AI weight maps to ScriptExpr

effect maps to BoundaryRequest, not EML

12.4 Tree Size Guard

large expression reports EML node count and depth

oversized expression rejected or warned

12.5 Designer Round Trip

designer formula

&#x20; → ScriptExpr

&#x20; → EML

&#x20; → CPU evaluate

&#x20; → explanation tree still references original terms

13\. Decision Rules



Use this checklist before lowering anything to EML.



Lower to EML only if all are true

\[ ] expression is pure

\[ ] expression is numeric

\[ ] all scopes are resolved

\[ ] no side effects

\[ ] no random behavior

\[ ] no event scheduling

\[ ] no structural mutation

\[ ] domain safety policy covers all operations

\[ ] compiled tree size is acceptable

\[ ] native SimThing primitives are insufficient or less appropriate

Do not lower to EML if any are true

\[ ] creates/removes/reparents SimThings

\[ ] activates/suspends overlays

\[ ] changes registry/dimensions

\[ ] depends on random selection

\[ ] performs scope traversal at runtime

\[ ] is simple Add/Multiply/Set overlay

\[ ] is simple threshold comparison

\[ ] is a designer metadata/localization construct

14\. Strategic Role of EML in SimThing



EML should be treated as a power tool, not a foundation replacement.



The foundation remains:



SimThing tree

DimensionRegistry

PropertyLayout

OverlayLifecycle

IntentDelta

ReductionRule

ThresholdRegistration

BoundaryProtocol

GPU dense matrices



EML can extend the system by making arbitrary numeric formulas more uniform and potentially GPU-friendly.



But SimThing should not become an EML engine.



The intended relationship is:



SimThing is the simulation architecture.

Script IR is the semantic expression layer.

EML is one possible numeric backend.

15\. Bottom Line



EML is promising for SimThing, especially for:



derived fields

AI risk frontiers

scripted values

composite trigger scores

designer-authored numeric formulas

symbolic-regression tooling



EML is not appropriate for:



effects

scope traversal

event scheduling

random lists

structural mutation

simple overlays

simple thresholds

designer-facing syntax



The correct integration plan is:



Clausewitz-like config

&#x20; → SimThing Script IR

&#x20; → native overlays / thresholds / boundary requests where possible

&#x20; → optional EML backend for pure numeric formulas



This keeps SimThing legible, debuggable, and GPU-forward without forcing every piece of game logic through a mathematically elegant but operationally expensive expression form.



The CPU understands the world.

The GPU evolves the world.

EML may help describe selected numeric fields inside the world.

