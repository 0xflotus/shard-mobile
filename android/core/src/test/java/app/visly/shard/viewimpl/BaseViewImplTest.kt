/**
 * Copyright (c) Visly Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

package app.visly.shard.viewimpl

import android.view.View
import androidx.test.core.app.ApplicationProvider
import app.visly.shard.JsonValue
import app.visly.shard.Color
import app.visly.shard.Size
import app.visly.shard.ShardContext
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.Before
import org.junit.runner.RunWith
import org.mockito.Mockito
import org.mockito.Mockito.verify
import org.robolectric.RobolectricTestRunner

@RunWith(RobolectricTestRunner::class)
class BaseViewImplTest {
    lateinit var viewImpl: BaseViewImpl<View>

    @Before
    fun setup() {
        val context = ShardContext(ApplicationProvider.getApplicationContext())
        viewImpl = object: BaseViewImpl<View>(context) {
            override fun measure(width: Float?, height: Float?): Size = Size(0f, 0f)
            override fun createView(): View = View(ctx)
        }
    }

    @Test
    fun testSetBackgroundColor() {
        viewImpl.setProp("background-color", JsonValue.String("#f00"))
        assertEquals(viewImpl.backgroundColor, Color(android.graphics.Color.RED, null))
    }

    @Test
    fun testSetBackgroundColorPressed() {
        viewImpl.setProp("background-color", JsonValue.Object(mapOf(
                "default" to JsonValue.String("#F00"),
                "pressed" to JsonValue.String("#00F")
        )))
        assertEquals(viewImpl.backgroundColor, Color(android.graphics.Color.RED, android.graphics.Color.BLUE))
    }

    @Test
    fun testSetBorderColor() {
        viewImpl.setProp("border-color", JsonValue.String("#f00"))
        assertEquals(viewImpl.borderColor, android.graphics.Color.RED)
    }

    @Test
    fun testSetBorderWidth() {
        viewImpl.setProp("border-width", JsonValue.Object(mapOf(
                "unit" to JsonValue.String("points"),
                "value" to JsonValue.Number(10f)
        )))
        assertEquals(viewImpl.borderWidth, 10f)
    }

    @Test
    fun testSetBorderRadius() {
        viewImpl.setProp("border-radius", JsonValue.Object(mapOf(
                "unit" to JsonValue.String("points"),
                "value" to JsonValue.Number(10f)
        )))
        assertEquals(viewImpl.borderRadius, 10f)
    }

    @Test
    fun testSetBorderRadiusMax() {
        viewImpl.setProp("border-radius", JsonValue.String("max"))
        assertEquals(viewImpl.borderRadius, Float.MAX_VALUE)
    }

    @Test
    fun testSetBackgroundForeground() {
        val view = Mockito.mock(View::class.java)
        viewImpl.bindView(view)
        verify(view).background = Mockito.any()
        verify(view).foreground = Mockito.any()
    }
}
