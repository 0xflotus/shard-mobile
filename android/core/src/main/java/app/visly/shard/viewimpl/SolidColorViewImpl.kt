/**
 * Copyright (c) Visly Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package app.visly.shard.viewimpl

import android.view.View
import app.visly.shard.Size
import app.visly.shard.ShardContext

class SolidColorViewImpl(ctx: ShardContext): BaseViewImpl<View>(ctx) {
    override fun measure(width: Float?, height: Float?): Size {
        return Size(width ?: 0f, height ?: 0f)
    }

    override fun createView(): View {
        return View(ctx)
    }
}