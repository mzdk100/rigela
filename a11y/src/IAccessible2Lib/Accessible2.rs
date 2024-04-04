/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use super::{
    AccessibleRelation::IAccessibleRelation,
    AccessibleStates::AccessibleStates,
    IA2CommonTypes::{IA2CoordinateType, IA2ScrollType},
};
use windows::core::interface;
use windows::core::BSTR;
use windows::core::HRESULT;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Accessibility::{IAccessible, IAccessible_Impl, IAccessible_Vtbl};

//noinspection SpellCheckingInspection
/**
 * @section _interfaces Interfaces
 * IAccessible2\n
 * IAccessible2_2\n
 * IAccessible2_3\n
 * IAccessibleAction\n
 * IAccessibleApplication\n
 * IAccessibleComponent\n
 * IAccessibleDocument\n
 * IAccessibleEditableText\n
 * IAccessibleHypertext\n
 * IAccessibleHypertext2\n
 * IAccessibleHyperlink\n
 * IAccessibleImage\n
 * IAccessibleRelation\n
 * IAccessibleTable (Deprecated)\n
 * IAccessibleTable2\n
 * IAccessibleTableCell\n
 * IAccessibleText\n
 * IAccessibleText2\n
 * IAccessibleValue
 *
 * @section _structs Structs
 * IA2Locale\n
 * IA2Range\n
 * IA2TableModelChange\n
 * IA2TextSegment
 *
 * @section _enums Enums
 * ::IA2Actions values are predefined actions for use when implementing support for HTML5 media.\n
 * ::IA2CoordinateType values define the requested coordinate type (screen or parent window).\n
 * ::IA2EventID values identify events.\n
 * ::IA2Role values defines roles which are in addition to the existing MSAA roles.\n
 * ::IA2ScrollType values define where to place an object or substring on the screen.\n
 * ::IA2States values define states which are in addition to the existing MSAA states.\n
 * ::IA2TableModelChangeType values describe the kinds of changes made to a table (insert, delete, update).\n
 * ::IA2TextBoundaryType values define the requested text unit (character, word, sentence, line, paragraph).\n
 * ::IA2TextSpecialOffsets values define special offsets for use in the text interfaces.
 *
 * @section _constants Constants
 * @ref grpRelations
 *
 * @section _misc Miscellaneous
 * @ref _licensePage "BSD License"\n
 * @ref _generalInfo "General Information"\n
 *
 * @page _licensePage BSD License
 * %IAccessible2 IDL Specification
 *
 * Copyright (c) 2007, 2013 Linux Foundation\n
 * Copyright (c) 2006 IBM Corporation\n
 * Copyright (c) 2000, 2006 Sun Microsystems, Inc.\n
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted if the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 * this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 * this list of conditions and the following disclaimer in the documentation and/or other materials
 * provided with the distribution.
 * 3. Neither the name of the Linux Foundation nor the names of its contributors may be used
 * to endorse
 * or promote products derived from this software without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 * CONTRIBUTORS "AS ARE" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED.
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 * OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 * EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 * This BSD License conforms to the Open Source Initiative "Simplified
 * BSD License" as published at:
 * <http://www.opensource.org/licenses/bsd-license.php>
 *
 * %IAccessible2 is a trademark of the Linux Foundation.
 * The %IAccessible2 mark may be used in accordance with
 * <a href="http://www.linuxfoundation.org/collaborate/workgroups/accessibility/trademark-policy">
 * Linux Foundation Trademark Policy</a>
 * to indicate compliance with the %IAccessible2 specification.
 *
 * @page _generalInfo General Information
 * The following information is applicable to two or more interfaces.
 *
 * @ref _errors\n
 * @ref _memory\n
 * &nbsp;&nbsp;@ref _arrayConsideration\n
 * @ref _indexes\n
 * @ref _enums\n
 * @ref _specialOffsets\n
 * @ref _discoveringInterfaces\n
 * @ref _changingInterfaces\n
 * @ref _applicationInfo\n
 * @ref _childIDs\n
 * @ref _variants\n
 * @ref _iaAction-iaHyperlink\n
 * @ref _trademark
 *
 * @section _errors Error Handling
 * HRESULT values are defined by the Microsoft&reg; Win32&reg; API.
 * For more information, refer to
 * <a href="http://msdn.microsoft.com/en-us/library/windows/desktop/aa378137%28v=vs.85%29.aspx"> Interpreting HRESULT Values</a> in MSDN&reg;.
 *
 * Note that the S_FALSE return value is considered a non-error value and the
 * SUCCEEDED macro will return TRUE.
 * S_FALSE is used when there is no failure,
 * but there was nothing valid to return, e.g.,
 * in IAccessible2::attributes when there are no attributes.
 * When S_FALSE is returned `[out]` pointer types should be NULL, and `[out]` longs should generally be 0,
 * but sometimes -1 is used such as IAccessible2::indexInParent,
 * IAccessibleText::caretOffset, and IAccessibleHypertext::hyperlinkIndex.
 *
 * Note that for BSTR `[out]` variables common COM practice is
 * that the server does the SysAllocString and the client does the SysFreeString.
 * Also note that when NULL is returned, there is no need for the client to call SysFreeString.
 * Please refer to the documentation for each method for more details regarding error handling.
 *
 * @section _memory Memory Management
 * The following memory management issues should be considered:
 * @li Although `[out]` BSTR variables are declared by the client,
 * their space is allocated by the server.
 * They need to be freed with SysFreeString by the client at the end of life;
 * the same is true when BSTRs are used in structs or arrays which are passed to the server.
 * @li If there is no valid `[out]` BSTR to return,
 * the server should return S_FALSE and assign NULL to the output,
 * e.g. `*theOutBSTR = NULL;`.
 * @li At the end of life,
 * COM interfaces need to be referenced with AddRef when used and dereferenced with Release.
 * @li Single `[out]` longs, HWNDs, booleans,
 * and structs are declared by the caller and passed by reference.
 * The marshaler does all the memory management.
 *
 * The following articles may be helpful for understanding memory management issues:
 * @li An article by Don Box in an
 *  <a href="http://www.microsoft.com/msj/1196/activex1196.aspx">Q & A section</a>
 * of the Microsoft Systems Journal's November 1996 edition.
 * @li A posting to a CodeGuru forum,
 *  <a href="http://www.codeguru.com/forum/showthread.php?t=364511">Windows SDK
 *  String: What are the rules for BSTR allocation and deallocation?</a>
 *
 * @subsection _arrayConsideration Special Consideration, when using Arrays,
 * There are several methods which return arrays.
 * In the case of IAccessible2::relations and IAccessibleRelation::targets the client must allocate and free the arrays.
 *
 * For the remaining methods which return arrays,
 * the server must allocate the array and the client must free the array when no longer needed.
 * These methods are
 * IAccessible2::extendedStates, IAccessible2::localizedExtendedStates,
 * IAccessible2_2::relationTargetsOfType,
 * IAccessibleAction::keyBinding, IAccessibleHypertext2::hyperlinks,
 * IAccessibleTable::selectedChildren, IAccessibleTable::selectedColumns,
 * IAccessibleTable::selectedRows, IAccessibleTable2::selectedCells,
 * IAccessibleTable2::selectedColumns,
 * IAccessibleTable2::selectedRows, IAccessibleTableCell::columnHeaderCells,
 * and IAccessibleTableCell::rowHeaderCells.
 * For those methods,
 * the server must allocate both the top level array and any storage associated with it,
 * e.g., for BSTRs.
 * The server must allocate the arrays with
 * CoTaskMemAlloc and any BSTRs with SysAllocString.
 * The client must use CoTaskMemFree to free the array,
 * and any BSTRs must be freed with SysFreeString.
 *
 * Also, the IDL for IAccessible2::extendedStates, IAccessible2::localizedExtendedStates,
 * IAccessibleAction::keyBinding, IAccessibleTable::selectedChildren,
 * IAccessibleTable::selectedColumns,
 * and IAccessibleTable::selectedRows includes an extraneous `[in]` parameter
 * for the caller to specify the max size of the array.
 * This parameter will be ignored by the COM server.
 *
 * @section _indexes Zero and One-Based Indexes
 * Unless otherwise specified, all offsets and indexes are 0 based.
 *
 * @section _enums Enums
 * Note that enums start at 0.
 *
 * @section _specialOffsets Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods IAccessibleText and IAccessibleEditableText can use one or more of the following special offset values.
 * They are defined in the IA2TextSpecialOffsets enum.
 * @li Using IA2_TEXT_OFFSET_LENGTH (-1)
 * as an offset in any of the IAccessibleText or IAccessibleEditableText methods is the same
 * as specifying the length of the string.
 * @li Using IA2_TEXT_OFFSET_CARET (-2) as an offset for IAccessibleText::textBeforeOffset,
 * IAccessibleText::textAtOffset,
 * and IAccessibleText::textAfterOffset indicates
 * that the text related to the physical location of the caret should be used.
 * This is needed for applications that consider the end's character offset of one line
 * (as reached by pressing the End key)
 * the same as the offset of the first character on the next line.
 *  Since the same offset is associated with two different lines,
 * a special means is needed to fetch text from the line where the caret is physically located.
 *
 * @section _discoveringInterfaces Discovery of Interfaces
 * In general AT (Assistive Technology) should try IAccessible2 interfaces,
 * followed by using the MSAA (Microsoft Active Accessibility) interfaces.
 * (In cases where the application is known to have custom interfaces which provide information not supplied by IAccessible2 or MSAA, then those custom interfaces can be used.)  The AT can then, by default, support unknown IAccessible2/MSAA applications, without the application developers having to request AT vendors for support on an individual application by application basis.
 *
 * When you have a reference to an IAccessible and require a reference to an IAccessible2,
 * use QueryService as follows:
 * ```c
 * // pAcc is a reference to the accessible object's IAccessible interface.
 * IServiceProvider *pService = NULL;
 * hr = pAcc->QueryInterface(IID_IServiceProvider, (void **)&pService);
 * if(SUCCEEDED(hr)) {
 *   IAccessible2 *pIA2 = NULL;
 *   hr = pService->QueryService(IID_IAccessible, IID_IAccessible2, (void**)&pIA2);
 *   if (SUCCEEDED(hr) && pIA2) {
 *     // The control supports IAccessible2.
 *     // pIA2 is the reference to the accessible object's IAccessible2 interface.
 *   }
 * }
 * ```
 *
 * @section _changingInterfaces Changing between Accessible Interfaces
 * Note that developers must always implement MSAA's IAccessible and,
 * if needed, some of the interfaces in the set of IAccessible2 interfaces.
 * Although the IAccessible2
 * IDL is coded such that IAccessible2 is a subclass of MSAA's IAccessible, none of
 * MSAA's IAccessible methods are redefined by IAccessible2.
 *
 * QueryService must be used
 * to switch from a reference to an MSAA IAccessible interface to another interface.
 * This has been
 * <a href="http://www.atia.org/files/public/Introducing_IAccessibleEx.doc">documented</a> and the pertinent facts have been extracted below:
 *
 * @par
 *  Why use QueryService instead of just using QueryInterface to get IAccessibleEx directly?
 * The reason is that since MSAA 2.0, clients don't talk to a server's
 *  IAccessible interface directly;
 * instead they talk to an intermediate MSAA-provided wrapper
 * that calls through the original IAccessible.
 * This wrapper provides services such as implementing IDispatch,
 * supplying information from MSAA 2.0's Dynamic Annotation service,
 * and scaling locations when running on Windows Vista with DPI scaling enabled.
 *  QueryService is the supported way to expose additional interfaces from an existing IAccessible
 * and was originally used by MSHTML to expose IHTMLElement objects corresponding to IAccessibles.
 * QueryService is often more convenient for servers to implement than QueryInterface.
 * Because it does not have the same requirements
 * for preserving object identity or symmetry/transitivity as QueryInterface,
 * so QueryService allows servers
 * to easily implement the interface on the same object or a separate object.
 * The latter is often hard to do with QueryInterface unless the original object supports aggregation.
 *
 * Two related references in MSDN&reg; are:
 * @li <a href="http://msdn.microsoft.com/en-us/library/ms696078(VS.85).aspx"> "Using QueryService to expose a native object model interface for an IAccessible object"</a>
 * @li <a href="http://msdn.microsoft.com/en-us/library/ms528415.aspx#acc_obj">"Accessing the Internet Explorer Object Associated with an Accessible Object"</a>
 *
 * Based on this information from Microsoft,
 * QueryService must be used
 * to switch back and forth between a reference to an MSAA IAccessible interface and any of the IAccessible2 interfaces.
 *
 * Regarding switching between any of the IAccessible2 interfaces,
 * applications implementing IAccessible2 should implement the IAccessible2 interfaces on a single object
 * since ATs will be using QueryInterface to switch between the IAccessible2 interfaces.
 * Implementing the IAccessible2 interfaces on separate objects would require the use of QueryService.
 * There is one exception,
 * IAccessibleApplication can be implemented on a separate object
 * so its common code doesn't have to be included in each accessible object.
 * ATs should use QueryService to access IAccessibleApplication.
 *
 * @section _applicationInfo Access to Information about the Application
 * Servers implementing IAccessible2 should provide access to the IAccessibleApplication interface via QueryService from any object
 * so that ATs can easily determine specific information about the application such as its name or version.
 *
 * @section _childIDs Child IDs
 * The IAccessible2 interfaces do not support child IDs, i.e., simple child elements.
 * Full accessible objects must be created for each object that supports IAccessible2.
 * Therefore,
 * MSAA's get_accChild should never return a child ID (other than CHILDID_SELF)
 * for an object that implements any of the IAccessible2 interfaces.
 *
 * Microsoft's UI Automation specification has the same limitation,
 * and this was resolved in the UI Automation Express specification
 * by adding IAccessibleEx::GetObjectForChild and IAccessibleEx::GetIAccessiblePair.
 * These methods allow mapping back and forth between an IAccessibleEx and an {IAccessible,
 * Child ID} pair.
 * A future version of IAccessible2 may include similar methods
 * to map back and forth between an IAccessible2 and an {IAccessible,
 * Child ID} pair.
 *
 * @section _variants VARIANTs
 * Some methods return a VARIANT.
 * Implementers need to make sure that the return type is specified, i.e., VT_I4, VT_IDISPATCH, etc.
 * The methods that return VARIANTs are IAccessibleHyperlink::anchor,
 * IAccessibleHyperlink::anchorTarget,
 * IAccessibleValue::currentValue, IAccessibleValue::maximumValue, IAccessibleValue::minimumValue.
 *
 * @section _iaaction-iahyperlink IAccessibleHyperlink as subclass of IAccessibleAction
 * In this version of the IDL, IAccessibleHyperlink is a subclass of IAccessibleAction.
 * However, there is no practical need for that inheritance,
 * and in some cases, such as an image map of smart tags,
 * it doesn't make sense because such an image map doesn't have actionable objects;
 * it's the secondary smart tags that are actionable.
 * As a result,
 * implementations should not rely on the inheritance
 * as it may be removed in a later version of the IDL.
 *
 * @section _trademark Trademark Attribution
 * The names of actual companies and products mentioned herein may be the trademarks of their respective owners.
 * In particular, Active Accessibility, Microsoft, MSDN,
 * and Win32 are trademarks of the Microsoft group of companies in the U.S.A. and/or other countries.
 *
 * */

/**
 * A structure defining the locale of an accessible object.
 * IAccessible2::locale returns this struct.
 * */
#[allow(dead_code)]
pub struct IA2Locale {
    language: BSTR,
    // ISO 639-1 Alpha-2 two character language code
    country: BSTR,
    // ISO 3166-1 Alpha-2 two character country codes
    variant: BSTR, // Application-specific variant of the locale
}

/**
 * This interface exposes the primary set of information about an
 * IAccessible2 enabled accessible object.
 * This interface must always be provided for objects
 * that support some portion of the collection of the %IAccessible2 interfaces.
 * Please refer to @ref _changingInterfaces "Changing between Accessible Interfaces"
 * for special considerations related to use of the MSAA IAccessible interface and the set of %IAccessible2 interfaces.
 * */
#[interface("E89F726E-C4F4-4c19-BB19-B647D7FA8478")]
pub(crate) unsafe trait IAccessible2: IAccessible {
    /**
     * Returns the number of accessible relations for this object.
     * `nRelations` */
    pub(crate) fn nRelations(&self, nRelations: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns one accessible relation for this object.
     * `relationIndex` 0 based
     * `relation` retrieval E_INVALIDARG if bad [in] passed
     * */
    pub(crate) fn relation(
        &self,
        relationIndex: i32,
        relation: *mut *mut IAccessibleRelation,
    ) -> HRESULT;

    /**
     * Returns multiple accessible relations for this object.
     * `maxRelations` maximum size of the array allocated by the client
     * `relations` The array of accessible relation objects.
     * Note that this array is to be allocated by the client and freed when no longer needed.
     * Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * `nRelations` actual number of relations in the returned array (not more than maxRelations).
     * retrieval S_FALSE if there are no relations, nRelations is set to 0.
     * note As a performant alternative,
     * client code should consider using IAccessible2_2::relationTargetsOfType.
     * */
    pub(crate) fn relations(
        &self,
        maxRelations: i32,
        relations: *mut *mut IAccessibleRelation,
        nRelations: *mut i32,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the role of an %IAccessible2 object.
     * `role` The role of an %IAccessible2 object.
     * @note
     * @li For convenience MSAA roles are also passed through this method,
     * so the AT doesn't have to also fetch roles through MSAA's get_accRole.
     * @li %IAccessible2 roles should not be passed through MSAA's get_accRole.
     * @li For compatibility with non IAccessible2 enabled ATs,
     * IAccessible2 applications should also add support to
     * get_accRole to return the closest MSAA role or ROLE_SYSTEM_CLIENT
     * (the MSAA defined default role) if there is not a good match.
     * @li This method is missing a [propget] prefix in the IDL.
     * The result is the method is named role in generated C++ code instead-of-get_role.
     * */
    pub(crate) fn role(&self, role: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Makes an object visible on the screen.
     * `scrollType` Defines where the object should be placed on the screen.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    pub(crate) fn scrollTo(&self, scrollType: IA2ScrollType) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Moves the top left of an object to a specified location.
     * `coordinateType` Specifies whether the coordinates are relative to the screen or the parent object.
     * `x` Defines the x coordinate.
     * `y` Defines the y coordinate.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    pub(crate) fn scrollToPoint(
        &self,
        coordinateType: IA2CoordinateType,
        x: i32,
        y: i32,
    ) -> HRESULT;

    /**
     * Returns grouping information.
     * Used for tree items, list items, tab panel labels, radio buttons, etc.
     * Also used for collections of non-text objects.
     * `groupLevel` 1 based, 0 indicates that this value is not applicable.
     * `similarItemsInGroup` 1 based, 0 indicates that this value is not applicable.
     * `positionInGroup` 1 based, 0 indicates that this value
     * is not applicable.
     * This is an index into the objects in the current group,
     * not an index into all the objects at the same group level.
     * If at least one value is valid.
     * retrieval S_FALSE if no values are valid, [out] values are 0s
     * @note This method is meant to describe the nature of an object's containment structure.
     * It's exposed by trees, tree grids, nested lists, nested menus, but not
     * headings, which uses the level object attribute.
     * It is also exposed by radio buttons (with groupLevel == 0).
     * @note This is normally not implemented on a combo box to describe the nature of its contents.
     * Normally, an AT will get that information from its child list object.
     * However, in some cases,
     * when non-edit combo boxes are not
     * able to structure themselves such that the list is a child of the combo box,
     * this method is implemented on the combo box itself.
     * ATs can use this interface if a child list is not found.
     * */
    pub(crate) fn groupPosition(
        &self,
        groupLevel: *mut i32,
        similarItemsInGroup: *mut i32,
        positionInGroup: *mut i32,
    ) -> HRESULT;

    /**
     * Returns the bit strip containing any IAccessible2 states.
     * The IAccessible2 states are in addition to the MSAA states and are defined in the IA2States enum.
     * `states` */
    pub(crate) fn states(&self, states: *mut AccessibleStates) -> HRESULT;

    /**
     * Returns the extended role.
     * An extended role is a role dynamically generated by the application.
     * It is not predefined by the %IAccessible2 specification.
     * `extendedRole` @retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    pub(crate) fn extendedRole(&self, extendedRole: *mut BSTR) -> HRESULT;

    /**
     * Returns the localized extended role.
     * `localizedExtendedRole` @retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    pub(crate) fn localizedExtendedRole(&self, localizedExtendedRole: *mut BSTR) -> HRESULT;

    /**
     * Returns the number of extended states.
     * `nExtendedStates` */
    pub(crate) fn nExtendedStates(&self, nExtendedStates: *mut i32) -> HRESULT;

    /**
     * Returns the extended states (array of strings).
     * An extended state is a state dynamically generated by the application.
     * It is not predefined by the %IAccessible2 specification.
     * `maxExtendedStates` This parameter is ignored.
     * Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * `extendedStates` This array is allocated by the server.
     * The client must free it with CoTaskMemFree.
     * `nExtendedStates` The number of extended states returned; the size of the returned array.
     * retrieval S_FALSE if there are no states, [out] values are NULL and 0 respectively
     * */
    pub(crate) fn extendedStates(
        &self,
        maxExtendedStates: i32,
        extendedStates: *mut *mut BSTR,
        nExtendedStates: *mut i32,
    ) -> HRESULT;

    /**
     * Returns the localized extended states (array of strings).
     * `maxLocalizedExtendedStates` This parameter is ignored.
     * Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * `localizedExtendedStates` This array is allocated by the server.
     * The client must free it with CoTaskMemFree.
     * `nLocalizedExtendedStates` The number of localized extended states returned;
     * the size of the returned array.
     * retrieval S_FALSE if there are no states, [out] values are NULL and 0 respectively
     * */
    pub(crate) fn localizedExtendedStates(
        &self,
        maxLocalizedExtendedStates: i32,
        localizedExtendedStates: *mut *mut BSTR,
        nLocalizedExtendedStates: *mut i32,
    ) -> HRESULT;

    //noinspection GrazieInspection
    /**
     * Returns the unique ID.
     * The uniqueID is an identifier for this object, is unique within the current window,
     * and remains the same for the lifetime of the accessible object.
     * The uniqueID is not related to:
     * - the MSAA objectID,
     * which is used by the server to disambiguate between IAccessibles per HWND or
     * - the MSAA childID,
     * which is used to disambiguate between children being managed by an IAccessible.
     * This value is provided
     * so the AT can have access to a unique runtime persistent identifier
     * even when not handling an event for the object.
     * An example of when this value is useful is if the AT wants to build a cache.
     * The AT could cache the uniqueIDs in addition to other data being cached.
     * When an event is triggered, the AT can map the uniqueId to its internal model.
     * Thus, if there's a REORDER/SHOW/HIDE event,
     * the AT knows which part of the internal structure has been invalidated
     * and can refetch just that part.
     * This value can also be used by an AT to determine when the current control has changed.
     * If the role is the same for two controls that are adjacent in the tab order,
     * this can be used to detect the new control.
     * Another use of this value by an AT is to identify when a grouping object has changed, e.g.,
     * when moving from a radio button in one group to a radio button in a different group.
     * One means of implementing this would be
     * to create a factory with a 32-bit number generator and a reuse pool.
     * The number generator would emit numbers starting at 1.
     * Each time an object's life cycle ended, its number would be saved into a reuse pool.
     * The number generator would be used whenever the reuse pool was empty.
     * Another way to create a unique ID is to generate it from a pointer value,
     * e.g., an object's address.
     * That would be unique because no two active objects can use the same allocated memory space.
     * `uniqueID` */
    pub(crate) fn uniqueID(&self, uniqueID: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the window handle for the parent window which contains this object.
     * This is the same window handle that will be passed for any events that occur on the object.
     * However, it is stored in the accessible object for use in situations
     * where it would be helpful to access the window handle,
     * even if no event is fired on this object.
     * A use case is when a screen reader is grabbing an entire web page on a page load.
     * To access the window handle,
     * the assistive technology (AT)
     * would have to use WindowFromAccessibleObject on each IAccessible.
     * It is slow
     * because it involves a loop that crawls up the ancestor chain
     * and searches for a ROLE_WINDOW object,
     * mapping it back to a window handle.
     * This is implemented by oleacc.dll.
     * However, with the availability of windowHandle, this process can be avoided.
     * `windowHandle` */
    pub(crate) fn windowHandle(&self, windowHandle: *mut HWND) -> HRESULT;

    /**
     * Returns the index of this object in its parent object.
     * `indexInParent` 0 based; -1 indicates there is no parent;
     * the upper bound is the value returned by the parent's IAccessible::get_accChildCount.
     * retrieval S_FALSE if no parent, [out] value is -1
     * */
    pub(crate) fn indexInParent(&self, indexInParent: *mut i32) -> HRESULT;

    /**
     * Returns the IA2Locale of the accessible object.
     * `locale` */
    pub(crate) fn locale(&self, locale: *mut IA2Locale) -> HRESULT;

    /**
     * Returns the attributes specific to this object, such as a cell's formula.
     * `attributes` @retrieval S_FALSE returned if there is nothing to return, [out] value is NULL
     * */
    pub(crate) fn attributes(&self, attributes: *mut BSTR) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (Accessible2.idl)
 *
 *  IAccessible2 IDL Specification
 *
 *  Copyright (c) 2007, 2013 Linux Foundation
 *  Copyright (c) 2006 IBM Corporation
 *  Copyright (c) 2000, 2006 Sun Microsystems, Inc.
 *  All rights reserved.
 *   
 *   
 *  Redistribution and use in source and binary forms, with or without
 *  modification, are permitted if the following conditions
 *  are met:
 *   
 *   1. Redistributions of source code must retain the above copyright
 *      notice, this list of conditions and the following disclaimer.
 *   
 *   2. Redistributions in binary form must reproduce the above
 *      copyright notice, this list of conditions and the following
 *      disclaimer in the documentation and/or other materials
 *      provided with the distribution.
 *
 *   3. Neither the name of the Linux Foundation nor the names of its
 *      contributors may be used to endorse or promote products
 *      derived from this software without specific prior written
 *      permission.
 *   
 *  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *  CONTRIBUTORS "AS ARE" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
 *  CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *  NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *  LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *  HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *  CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *  OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *  EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *   
 *  This BSD License conforms to the Open Source Initiative "Simplified
 *  BSD License" as published at:
 *  <http://www.opensource.org/licenses/bsd-license.php>
 *   
 *  IAccessible2 is a trademark of the Linux Foundation. The IAccessible2
 *  mark may be used in accordance with the Linux Foundation Trademark
 *  Policy to indicate compliance with the IAccessible2 specification.
 * */
trait IdlCopyright {}
