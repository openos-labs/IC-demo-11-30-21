import HashMap "mo:base/HashMap";
import Iter "mo:base/Iter";
import Nat64 "mo:base/Nat64";
import Nat "mo:base/Nat";
import Principal "mo:base/Principal";
import Text "mo:base/Text";

shared(msg) actor class Demo() {
    type HeaderField = (Text, Text);
    type HttpResponse = {
        status_code: Nat16;
        headers: [HeaderField];
        body: Blob;
    };

    private var balances = HashMap.HashMap<Principal, Nat64>(0, Principal.equal, Principal.hash);
    private stable var balanceEntries : [(Principal, Nat64)] = [];

    balances.put(msg.caller, 1000);

    private func _balanceOf(who: Principal) : Nat64 {
        switch (balances.get(who)) {
            case (?balance) {
                balance
            };
            case (null) {
                0
            };
        };
    };

    public shared(msg) func transfer(to: Principal, value: Nat64) : async Bool {
        let from_balance = _balanceOf(msg.caller);
        let from_balance_new = from_balance - value;
        if (from_balance_new != 0) {
            balances.put(msg.caller, from_balance_new);
        } else {
            balances.delete(msg.caller);
        };

        let to_balance = _balanceOf(to);
        let to_balance_new : Nat64 = to_balance + value;
        if (to_balance_new != 0) {
            balances.put(to, to_balance_new);
        };
        true
    };

    public query func balanceOf(who: Principal) : async Nat64 {
        _balanceOf(who)
    };

    public query func allBalances() : async [(Principal, Nat64)] {
        Iter.toArray(balances.entries());
    };

    public query func http_request() : async HttpResponse {
        var list = "Total " # Nat.toText(balances.size()) # " hodl: \n\n" # "Principal:                                                       balances: \n";
        for ((k,v) in balances.entries()) {
            list := list # Principal.toText(k) # "  " # Nat64.toText(v) # "\n";
        };
        {
            status_code = 200;
            headers = [("content-type", "text/plain")];
            body = Text.encodeUtf8 (list)
        }
    };

    system func preupgrade() {
        balanceEntries := Iter.toArray(balances.entries());
    };

    system func postupgrade() {
        balances := HashMap.fromIter<Principal, Nat64>(balanceEntries.vals(), 1, Principal.equal, Principal.hash);
    };
};
