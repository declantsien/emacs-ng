// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((guint) ATK_HYPERLINK_IS_INLINE);
    PRINT_CONSTANT((gint) ATK_KEY_EVENT_LAST_DEFINED);
    PRINT_CONSTANT((gint) ATK_KEY_EVENT_PRESS);
    PRINT_CONSTANT((gint) ATK_KEY_EVENT_RELEASE);
    PRINT_CONSTANT((gint) ATK_LAYER_BACKGROUND);
    PRINT_CONSTANT((gint) ATK_LAYER_CANVAS);
    PRINT_CONSTANT((gint) ATK_LAYER_INVALID);
    PRINT_CONSTANT((gint) ATK_LAYER_MDI);
    PRINT_CONSTANT((gint) ATK_LAYER_OVERLAY);
    PRINT_CONSTANT((gint) ATK_LAYER_POPUP);
    PRINT_CONSTANT((gint) ATK_LAYER_WIDGET);
    PRINT_CONSTANT((gint) ATK_LAYER_WINDOW);
    PRINT_CONSTANT((gint) ATK_RELATION_CONTROLLED_BY);
    PRINT_CONSTANT((gint) ATK_RELATION_CONTROLLER_FOR);
    PRINT_CONSTANT((gint) ATK_RELATION_DESCRIBED_BY);
    PRINT_CONSTANT((gint) ATK_RELATION_DESCRIPTION_FOR);
    PRINT_CONSTANT((gint) ATK_RELATION_DETAILS);
    PRINT_CONSTANT((gint) ATK_RELATION_DETAILS_FOR);
    PRINT_CONSTANT((gint) ATK_RELATION_EMBEDDED_BY);
    PRINT_CONSTANT((gint) ATK_RELATION_EMBEDS);
    PRINT_CONSTANT((gint) ATK_RELATION_ERROR_FOR);
    PRINT_CONSTANT((gint) ATK_RELATION_ERROR_MESSAGE);
    PRINT_CONSTANT((gint) ATK_RELATION_FLOWS_FROM);
    PRINT_CONSTANT((gint) ATK_RELATION_FLOWS_TO);
    PRINT_CONSTANT((gint) ATK_RELATION_LABELLED_BY);
    PRINT_CONSTANT((gint) ATK_RELATION_LABEL_FOR);
    PRINT_CONSTANT((gint) ATK_RELATION_LAST_DEFINED);
    PRINT_CONSTANT((gint) ATK_RELATION_MEMBER_OF);
    PRINT_CONSTANT((gint) ATK_RELATION_NODE_CHILD_OF);
    PRINT_CONSTANT((gint) ATK_RELATION_NODE_PARENT_OF);
    PRINT_CONSTANT((gint) ATK_RELATION_NULL);
    PRINT_CONSTANT((gint) ATK_RELATION_PARENT_WINDOW_OF);
    PRINT_CONSTANT((gint) ATK_RELATION_POPUP_FOR);
    PRINT_CONSTANT((gint) ATK_RELATION_SUBWINDOW_OF);
    PRINT_CONSTANT((gint) ATK_ROLE_ACCEL_LABEL);
    PRINT_CONSTANT((gint) ATK_ROLE_ALERT);
    PRINT_CONSTANT((gint) ATK_ROLE_ANIMATION);
    PRINT_CONSTANT((gint) ATK_ROLE_APPLICATION);
    PRINT_CONSTANT((gint) ATK_ROLE_ARROW);
    PRINT_CONSTANT((gint) ATK_ROLE_ARTICLE);
    PRINT_CONSTANT((gint) ATK_ROLE_AUDIO);
    PRINT_CONSTANT((gint) ATK_ROLE_AUTOCOMPLETE);
    PRINT_CONSTANT((gint) ATK_ROLE_BLOCK_QUOTE);
    PRINT_CONSTANT((gint) ATK_ROLE_CALENDAR);
    PRINT_CONSTANT((gint) ATK_ROLE_CANVAS);
    PRINT_CONSTANT((gint) ATK_ROLE_CAPTION);
    PRINT_CONSTANT((gint) ATK_ROLE_CHART);
    PRINT_CONSTANT((gint) ATK_ROLE_CHECK_BOX);
    PRINT_CONSTANT((gint) ATK_ROLE_CHECK_MENU_ITEM);
    PRINT_CONSTANT((gint) ATK_ROLE_COLOR_CHOOSER);
    PRINT_CONSTANT((gint) ATK_ROLE_COLUMN_HEADER);
    PRINT_CONSTANT((gint) ATK_ROLE_COMBO_BOX);
    PRINT_CONSTANT((gint) ATK_ROLE_COMMENT);
    PRINT_CONSTANT((gint) ATK_ROLE_CONTENT_DELETION);
    PRINT_CONSTANT((gint) ATK_ROLE_CONTENT_INSERTION);
    PRINT_CONSTANT((gint) ATK_ROLE_DATE_EDITOR);
    PRINT_CONSTANT((gint) ATK_ROLE_DEFINITION);
    PRINT_CONSTANT((gint) ATK_ROLE_DESCRIPTION_LIST);
    PRINT_CONSTANT((gint) ATK_ROLE_DESCRIPTION_TERM);
    PRINT_CONSTANT((gint) ATK_ROLE_DESCRIPTION_VALUE);
    PRINT_CONSTANT((gint) ATK_ROLE_DESKTOP_FRAME);
    PRINT_CONSTANT((gint) ATK_ROLE_DESKTOP_ICON);
    PRINT_CONSTANT((gint) ATK_ROLE_DIAL);
    PRINT_CONSTANT((gint) ATK_ROLE_DIALOG);
    PRINT_CONSTANT((gint) ATK_ROLE_DIRECTORY_PANE);
    PRINT_CONSTANT((gint) ATK_ROLE_DOCUMENT_EMAIL);
    PRINT_CONSTANT((gint) ATK_ROLE_DOCUMENT_FRAME);
    PRINT_CONSTANT((gint) ATK_ROLE_DOCUMENT_PRESENTATION);
    PRINT_CONSTANT((gint) ATK_ROLE_DOCUMENT_SPREADSHEET);
    PRINT_CONSTANT((gint) ATK_ROLE_DOCUMENT_TEXT);
    PRINT_CONSTANT((gint) ATK_ROLE_DOCUMENT_WEB);
    PRINT_CONSTANT((gint) ATK_ROLE_DRAWING_AREA);
    PRINT_CONSTANT((gint) ATK_ROLE_EDITBAR);
    PRINT_CONSTANT((gint) ATK_ROLE_EMBEDDED);
    PRINT_CONSTANT((gint) ATK_ROLE_ENTRY);
    PRINT_CONSTANT((gint) ATK_ROLE_FILE_CHOOSER);
    PRINT_CONSTANT((gint) ATK_ROLE_FILLER);
    PRINT_CONSTANT((gint) ATK_ROLE_FONT_CHOOSER);
    PRINT_CONSTANT((gint) ATK_ROLE_FOOTER);
    PRINT_CONSTANT((gint) ATK_ROLE_FOOTNOTE);
    PRINT_CONSTANT((gint) ATK_ROLE_FORM);
    PRINT_CONSTANT((gint) ATK_ROLE_FRAME);
    PRINT_CONSTANT((gint) ATK_ROLE_GLASS_PANE);
    PRINT_CONSTANT((gint) ATK_ROLE_GROUPING);
    PRINT_CONSTANT((gint) ATK_ROLE_HEADER);
    PRINT_CONSTANT((gint) ATK_ROLE_HEADING);
    PRINT_CONSTANT((gint) ATK_ROLE_HTML_CONTAINER);
    PRINT_CONSTANT((gint) ATK_ROLE_ICON);
    PRINT_CONSTANT((gint) ATK_ROLE_IMAGE);
    PRINT_CONSTANT((gint) ATK_ROLE_IMAGE_MAP);
    PRINT_CONSTANT((gint) ATK_ROLE_INFO_BAR);
    PRINT_CONSTANT((gint) ATK_ROLE_INPUT_METHOD_WINDOW);
    PRINT_CONSTANT((gint) ATK_ROLE_INTERNAL_FRAME);
    PRINT_CONSTANT((gint) ATK_ROLE_INVALID);
    PRINT_CONSTANT((gint) ATK_ROLE_LABEL);
    PRINT_CONSTANT((gint) ATK_ROLE_LANDMARK);
    PRINT_CONSTANT((gint) ATK_ROLE_LAST_DEFINED);
    PRINT_CONSTANT((gint) ATK_ROLE_LAYERED_PANE);
    PRINT_CONSTANT((gint) ATK_ROLE_LEVEL_BAR);
    PRINT_CONSTANT((gint) ATK_ROLE_LINK);
    PRINT_CONSTANT((gint) ATK_ROLE_LIST);
    PRINT_CONSTANT((gint) ATK_ROLE_LIST_BOX);
    PRINT_CONSTANT((gint) ATK_ROLE_LIST_ITEM);
    PRINT_CONSTANT((gint) ATK_ROLE_LOG);
    PRINT_CONSTANT((gint) ATK_ROLE_MARK);
    PRINT_CONSTANT((gint) ATK_ROLE_MARQUEE);
    PRINT_CONSTANT((gint) ATK_ROLE_MATH);
    PRINT_CONSTANT((gint) ATK_ROLE_MATH_FRACTION);
    PRINT_CONSTANT((gint) ATK_ROLE_MATH_ROOT);
    PRINT_CONSTANT((gint) ATK_ROLE_MENU);
    PRINT_CONSTANT((gint) ATK_ROLE_MENU_BAR);
    PRINT_CONSTANT((gint) ATK_ROLE_MENU_ITEM);
    PRINT_CONSTANT((gint) ATK_ROLE_NOTIFICATION);
    PRINT_CONSTANT((gint) ATK_ROLE_OPTION_PANE);
    PRINT_CONSTANT((gint) ATK_ROLE_PAGE);
    PRINT_CONSTANT((gint) ATK_ROLE_PAGE_TAB);
    PRINT_CONSTANT((gint) ATK_ROLE_PAGE_TAB_LIST);
    PRINT_CONSTANT((gint) ATK_ROLE_PANEL);
    PRINT_CONSTANT((gint) ATK_ROLE_PARAGRAPH);
    PRINT_CONSTANT((gint) ATK_ROLE_PASSWORD_TEXT);
    PRINT_CONSTANT((gint) ATK_ROLE_POPUP_MENU);
    PRINT_CONSTANT((gint) ATK_ROLE_PROGRESS_BAR);
    PRINT_CONSTANT((gint) ATK_ROLE_PUSH_BUTTON);
    PRINT_CONSTANT((gint) ATK_ROLE_PUSH_BUTTON_MENU);
    PRINT_CONSTANT((gint) ATK_ROLE_RADIO_BUTTON);
    PRINT_CONSTANT((gint) ATK_ROLE_RADIO_MENU_ITEM);
    PRINT_CONSTANT((gint) ATK_ROLE_RATING);
    PRINT_CONSTANT((gint) ATK_ROLE_REDUNDANT_OBJECT);
    PRINT_CONSTANT((gint) ATK_ROLE_ROOT_PANE);
    PRINT_CONSTANT((gint) ATK_ROLE_ROW_HEADER);
    PRINT_CONSTANT((gint) ATK_ROLE_RULER);
    PRINT_CONSTANT((gint) ATK_ROLE_SCROLL_BAR);
    PRINT_CONSTANT((gint) ATK_ROLE_SCROLL_PANE);
    PRINT_CONSTANT((gint) ATK_ROLE_SECTION);
    PRINT_CONSTANT((gint) ATK_ROLE_SEPARATOR);
    PRINT_CONSTANT((gint) ATK_ROLE_SLIDER);
    PRINT_CONSTANT((gint) ATK_ROLE_SPIN_BUTTON);
    PRINT_CONSTANT((gint) ATK_ROLE_SPLIT_PANE);
    PRINT_CONSTANT((gint) ATK_ROLE_STATIC);
    PRINT_CONSTANT((gint) ATK_ROLE_STATUSBAR);
    PRINT_CONSTANT((gint) ATK_ROLE_SUBSCRIPT);
    PRINT_CONSTANT((gint) ATK_ROLE_SUGGESTION);
    PRINT_CONSTANT((gint) ATK_ROLE_SUPERSCRIPT);
    PRINT_CONSTANT((gint) ATK_ROLE_TABLE);
    PRINT_CONSTANT((gint) ATK_ROLE_TABLE_CELL);
    PRINT_CONSTANT((gint) ATK_ROLE_TABLE_COLUMN_HEADER);
    PRINT_CONSTANT((gint) ATK_ROLE_TABLE_ROW);
    PRINT_CONSTANT((gint) ATK_ROLE_TABLE_ROW_HEADER);
    PRINT_CONSTANT((gint) ATK_ROLE_TEAR_OFF_MENU_ITEM);
    PRINT_CONSTANT((gint) ATK_ROLE_TERMINAL);
    PRINT_CONSTANT((gint) ATK_ROLE_TEXT);
    PRINT_CONSTANT((gint) ATK_ROLE_TIMER);
    PRINT_CONSTANT((gint) ATK_ROLE_TITLE_BAR);
    PRINT_CONSTANT((gint) ATK_ROLE_TOGGLE_BUTTON);
    PRINT_CONSTANT((gint) ATK_ROLE_TOOL_BAR);
    PRINT_CONSTANT((gint) ATK_ROLE_TOOL_TIP);
    PRINT_CONSTANT((gint) ATK_ROLE_TREE);
    PRINT_CONSTANT((gint) ATK_ROLE_TREE_ITEM);
    PRINT_CONSTANT((gint) ATK_ROLE_TREE_TABLE);
    PRINT_CONSTANT((gint) ATK_ROLE_UNKNOWN);
    PRINT_CONSTANT((gint) ATK_ROLE_VIDEO);
    PRINT_CONSTANT((gint) ATK_ROLE_VIEWPORT);
    PRINT_CONSTANT((gint) ATK_ROLE_WINDOW);
    PRINT_CONSTANT((gint) ATK_SCROLL_ANYWHERE);
    PRINT_CONSTANT((gint) ATK_SCROLL_BOTTOM_EDGE);
    PRINT_CONSTANT((gint) ATK_SCROLL_BOTTOM_RIGHT);
    PRINT_CONSTANT((gint) ATK_SCROLL_LEFT_EDGE);
    PRINT_CONSTANT((gint) ATK_SCROLL_RIGHT_EDGE);
    PRINT_CONSTANT((gint) ATK_SCROLL_TOP_EDGE);
    PRINT_CONSTANT((gint) ATK_SCROLL_TOP_LEFT);
    PRINT_CONSTANT((gint) ATK_STATE_ACTIVE);
    PRINT_CONSTANT((gint) ATK_STATE_ANIMATED);
    PRINT_CONSTANT((gint) ATK_STATE_ARMED);
    PRINT_CONSTANT((gint) ATK_STATE_BUSY);
    PRINT_CONSTANT((gint) ATK_STATE_CHECKABLE);
    PRINT_CONSTANT((gint) ATK_STATE_CHECKED);
    PRINT_CONSTANT((gint) ATK_STATE_COLLAPSED);
    PRINT_CONSTANT((gint) ATK_STATE_DEFAULT);
    PRINT_CONSTANT((gint) ATK_STATE_DEFUNCT);
    PRINT_CONSTANT((gint) ATK_STATE_EDITABLE);
    PRINT_CONSTANT((gint) ATK_STATE_ENABLED);
    PRINT_CONSTANT((gint) ATK_STATE_EXPANDABLE);
    PRINT_CONSTANT((gint) ATK_STATE_EXPANDED);
    PRINT_CONSTANT((gint) ATK_STATE_FOCUSABLE);
    PRINT_CONSTANT((gint) ATK_STATE_FOCUSED);
    PRINT_CONSTANT((gint) ATK_STATE_HAS_POPUP);
    PRINT_CONSTANT((gint) ATK_STATE_HAS_TOOLTIP);
    PRINT_CONSTANT((gint) ATK_STATE_HORIZONTAL);
    PRINT_CONSTANT((gint) ATK_STATE_ICONIFIED);
    PRINT_CONSTANT((gint) ATK_STATE_INDETERMINATE);
    PRINT_CONSTANT((gint) ATK_STATE_INVALID);
    PRINT_CONSTANT((gint) ATK_STATE_INVALID_ENTRY);
    PRINT_CONSTANT((gint) ATK_STATE_LAST_DEFINED);
    PRINT_CONSTANT((gint) ATK_STATE_MANAGES_DESCENDANTS);
    PRINT_CONSTANT((gint) ATK_STATE_MODAL);
    PRINT_CONSTANT((gint) ATK_STATE_MULTISELECTABLE);
    PRINT_CONSTANT((gint) ATK_STATE_MULTI_LINE);
    PRINT_CONSTANT((gint) ATK_STATE_OPAQUE);
    PRINT_CONSTANT((gint) ATK_STATE_PRESSED);
    PRINT_CONSTANT((gint) ATK_STATE_READ_ONLY);
    PRINT_CONSTANT((gint) ATK_STATE_REQUIRED);
    PRINT_CONSTANT((gint) ATK_STATE_RESIZABLE);
    PRINT_CONSTANT((gint) ATK_STATE_SELECTABLE);
    PRINT_CONSTANT((gint) ATK_STATE_SELECTABLE_TEXT);
    PRINT_CONSTANT((gint) ATK_STATE_SELECTED);
    PRINT_CONSTANT((gint) ATK_STATE_SENSITIVE);
    PRINT_CONSTANT((gint) ATK_STATE_SHOWING);
    PRINT_CONSTANT((gint) ATK_STATE_SINGLE_LINE);
    PRINT_CONSTANT((gint) ATK_STATE_STALE);
    PRINT_CONSTANT((gint) ATK_STATE_SUPPORTS_AUTOCOMPLETION);
    PRINT_CONSTANT((gint) ATK_STATE_TRANSIENT);
    PRINT_CONSTANT((gint) ATK_STATE_TRUNCATED);
    PRINT_CONSTANT((gint) ATK_STATE_VERTICAL);
    PRINT_CONSTANT((gint) ATK_STATE_VISIBLE);
    PRINT_CONSTANT((gint) ATK_STATE_VISITED);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_BG_COLOR);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_BG_FULL_HEIGHT);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_BG_STIPPLE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_DIRECTION);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_EDITABLE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_FAMILY_NAME);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_FG_COLOR);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_FG_STIPPLE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_INDENT);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_INVALID);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_INVISIBLE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_JUSTIFICATION);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_LANGUAGE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_LAST_DEFINED);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_LEFT_MARGIN);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_PIXELS_ABOVE_LINES);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_PIXELS_BELOW_LINES);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_PIXELS_INSIDE_WRAP);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_RIGHT_MARGIN);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_RISE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_SCALE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_SIZE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_STRETCH);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_STRIKETHROUGH);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_STYLE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_TEXT_POSITION);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_UNDERLINE);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_VARIANT);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_WEIGHT);
    PRINT_CONSTANT((gint) ATK_TEXT_ATTR_WRAP_MODE);
    PRINT_CONSTANT((gint) ATK_TEXT_BOUNDARY_CHAR);
    PRINT_CONSTANT((gint) ATK_TEXT_BOUNDARY_LINE_END);
    PRINT_CONSTANT((gint) ATK_TEXT_BOUNDARY_LINE_START);
    PRINT_CONSTANT((gint) ATK_TEXT_BOUNDARY_SENTENCE_END);
    PRINT_CONSTANT((gint) ATK_TEXT_BOUNDARY_SENTENCE_START);
    PRINT_CONSTANT((gint) ATK_TEXT_BOUNDARY_WORD_END);
    PRINT_CONSTANT((gint) ATK_TEXT_BOUNDARY_WORD_START);
    PRINT_CONSTANT((gint) ATK_TEXT_CLIP_BOTH);
    PRINT_CONSTANT((gint) ATK_TEXT_CLIP_MAX);
    PRINT_CONSTANT((gint) ATK_TEXT_CLIP_MIN);
    PRINT_CONSTANT((gint) ATK_TEXT_CLIP_NONE);
    PRINT_CONSTANT((gint) ATK_TEXT_GRANULARITY_CHAR);
    PRINT_CONSTANT((gint) ATK_TEXT_GRANULARITY_LINE);
    PRINT_CONSTANT((gint) ATK_TEXT_GRANULARITY_PARAGRAPH);
    PRINT_CONSTANT((gint) ATK_TEXT_GRANULARITY_SENTENCE);
    PRINT_CONSTANT((gint) ATK_TEXT_GRANULARITY_WORD);
    PRINT_CONSTANT((gint) ATK_VALUE_ACCEPTABLE);
    PRINT_CONSTANT((gint) ATK_VALUE_BAD);
    PRINT_CONSTANT((gint) ATK_VALUE_BEST);
    PRINT_CONSTANT((gint) ATK_VALUE_GOOD);
    PRINT_CONSTANT((gint) ATK_VALUE_HIGH);
    PRINT_CONSTANT((gint) ATK_VALUE_LAST_DEFINED);
    PRINT_CONSTANT((gint) ATK_VALUE_LOW);
    PRINT_CONSTANT((gint) ATK_VALUE_MEDIUM);
    PRINT_CONSTANT((gint) ATK_VALUE_STRONG);
    PRINT_CONSTANT((gint) ATK_VALUE_VERY_BAD);
    PRINT_CONSTANT((gint) ATK_VALUE_VERY_GOOD);
    PRINT_CONSTANT((gint) ATK_VALUE_VERY_HIGH);
    PRINT_CONSTANT((gint) ATK_VALUE_VERY_LOW);
    PRINT_CONSTANT((gint) ATK_VALUE_VERY_STRONG);
    PRINT_CONSTANT((gint) ATK_VALUE_VERY_WEAK);
    PRINT_CONSTANT((gint) ATK_VALUE_WEAK);
    PRINT_CONSTANT((gint) ATK_XY_PARENT);
    PRINT_CONSTANT((gint) ATK_XY_SCREEN);
    PRINT_CONSTANT((gint) ATK_XY_WINDOW);
    return 0;
}
