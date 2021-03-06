/**
 * Copyright (c) Visly Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Quick
import Nimble
@testable import ShardKit

class ShardViewManagerSpec: QuickSpec {
    override func spec() {
        it("has correct defaults") {
            expect(ShardViewManager.shared.implFactories.count).to(equal(5))
            expect(ShardViewManager.shared.implFactories.keys).to(contain("flexbox"))
            expect(ShardViewManager.shared.implFactories.keys).to(contain("image"))
            expect(ShardViewManager.shared.implFactories.keys).to(contain("text"))
            expect(ShardViewManager.shared.implFactories.keys).to(contain("scroll"))
            expect(ShardViewManager.shared.implFactories.keys).to(contain("solid-color"))
        }
    }
}
