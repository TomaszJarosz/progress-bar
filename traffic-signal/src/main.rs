//https://zerotomastery.io/blog/rust-typestate-patterns/

trait SignalState {}


struct Red;

struct Yellow;

struct Green;

struct Fault;

impl SignalState for Red {}

impl SignalState for Yellow {}

impl SignalState for Green {}

impl SignalState for Fault {}

use std::marker::PhantomData;

struct TrafficSignal<S: SignalState> {
    _marker: PhantomData<S>,
}

impl<S: SignalState> TrafficSignal<S> {
    fn transition() -> TrafficSignal<S> {
        TrafficSignal {
            _marker: PhantomData,
        }
    }

    pub fn fault(self) -> TrafficSignal<Fault> {
        TrafficSignal::transition()
    }
}

impl TrafficSignal<Red> {
    pub fn next(self) -> TrafficSignal<Green> {
        TrafficSignal::transition()
    }
}

impl TrafficSignal<Green> {
    pub fn next(self) -> TrafficSignal<Yellow> {
        TrafficSignal::transition()
    }
}

impl TrafficSignal<Yellow> {
    pub fn next(self) -> TrafficSignal<Red> {
        TrafficSignal::transition()
    }
}

impl TrafficSignal<Fault> {
    pub fn clear_fault(self) -> TrafficSignal<Red> {
        TrafficSignal::transition()
    }

    pub fn initial() -> TrafficSignal<Fault> {
        TrafficSignal::transition()
    }
}

fn main() {
    let signal: TrafficSignal<Fault> = TrafficSignal::initial();
    let signal: TrafficSignal<Red> = signal.clear_fault();
    let signal: TrafficSignal<Green> = signal.next();
    let signal: TrafficSignal<Yellow> = signal.next();
    let signal: TrafficSignal<Red> = signal.next();
    let signal: TrafficSignal<Green> = signal.next();
    let _signal: TrafficSignal<Yellow> = signal.next();
}
