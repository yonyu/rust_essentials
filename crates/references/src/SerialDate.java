import javax.lang.model.element.Element;

/* An abstract class that defined our requirements for manipulating dates,
 * without tying down a particular implementation.
 * <P>
 * Requirement 1: match at least what Excel does for dates;
 * Requirement 2: class is immutable;
 * <P>
 * Why not just use java.util.Date? We will, when it makes sense. At times,
 * java.util.Date can be too precise - it represents an instant in time,
 * accurate to 1/1000th of a second (with the date itself depending on the
 * time-zone). Sometimes we just want to represent a particular day (e.g. 21
 * January 2015) without concerning ourselves about the time of day, or the
 * time-zone, or anything else. That's what we've defined SerialDate for.
 * <P>
 * You can call getInstance() to get a concrete subclass of SerialDate,
 * without worrying about the exact implementation.
 *
 * @author David Gilbert
 */
public abstract class SerialDate {

    /* Useful range constant. */
    public static final int INCLUDE_NONE = 0;
    /* Useful range constant. */
    public static final int INCLUDE_FIRST = 1;
    /* Useful range constant. */
    public static final int INCLUDE_SECOND = 2;
    /* Useful range constant. */
    public static final int INCLUDE_BOTH = 3;

    /**
     * Returns the serial number for the date, where 1 January 1900 = 2 (this
     * corresponds, almost, to the numbering system used in Microsoft Excel for
     * Windows and Lotus 1-2-3).
     *
     * @return the serial number for the date
     */
    public abstract int toSerial();

    public static SerialDate addMonths(final int months,
                                       final SerialDate base) {
        final int yy = (12 * base.getYYYY() +
                base.getMonth() + months - 1 ) / 12;
        final int mm  = (12 * base.getYYYY() +
                base.getMonth() + months - 1 ) % 12 + 1;
        final int dd = Math.min(
                base.getDayOfMonth(),
                SerialDate.lastDayOfMonth(mm, yy));

        return SerialDate.createInstance(dd, mm, yy);
    }

    public static CalendarDate addMonths(int monthsToAdd,
                                          CalendarDate base) {
        int baseYear = base.getYear();
        int baseMonth = base.getMonth() - JANUARY;
        int monthsInBaseYear = 12 * baseYear;
        int baseMonths = baseMonth + monthsInBaseYear;
        int resultMonths = baseMonths + monthsToAdd;
        int resultYear = resultMonths / 12;
        int resultMonth = resultMonths % 12 + JANUARY;
        int resultDay = Math.min(
                base.getDayOfMonth(),
                CalendarDate.lastDayOfMonth(resultMonth, resultYear));

        return CalendarDate.createInstance(
                resultDay, resultMonth, resultYear);
    }

    public boolean isInRange(final SerialDate d1,
                             final SerialDate d2,
                             final int include) {
        final int s1 = d1.toSerial();
        final int s2 = d2.toSerial();
        final int start = Math.min(s1, s2);
        final int end = Math.max(s1, s2);

        final int s = toSerial();
        if (include == INCLUDE_BOTH) {
            return s >= start && s <= end;
        }
        else if (include == INCLUDE_FIRST) {
            return s >= start && s < end;
        }
        else if (include == INCLUDE_SECOND) {
            return s > start && s <= end;
        } else {
            return s > start && s < end;
        }
    }

    public createElementsInDocument() {
    Document d = docBuilder.newDocument();
    Element rootElement = d.createElement(XMLConstants.TESTSUITE);
    rootElement.setAttribute(XMLConstants.ATTR_NAME, m_testContext.getName());

    Element propsElement = d.createElement(XMLConstants.PROPERTIES);
    rootElement.appendChild(propsElement);

    ISuite suite = m_testContext.getSuite();

    rootElement.setAttribute(XMLConstants.ATTR_NAME, "" + m_allTests.size());
    rootElement.setAttribute(XMLConstants.ATTR_NAME, "" + m_numFailed);
    rootElement.setAttribute(XMLConstants.ATTR_NAME, "0");

    long elapsedTimeMillis = m_testContext.getEndDate().getTime() - m_testContext.getStartDate().getTime();
    rootElement.setAttribute(XMLConstants.ATTR_NAME, "" + elapsedTimeMillis / 1000);
    }

    /**
     *
     * @param shortened a flag indicating that shortened month names should
     *                  be returned.
     * @return an array of month names
     */
    public static String[] getMonths(final boolean shortened) {




        if (shortened) {
            return DATE_FORMAT_SYMBOLS.getShortMonths();
        } else {
            return DATE_FORMAT_SYMBOLS.getMonths();
        }
    }

    public static byte[] encode(byte[] binaryData)
    {
        int lengthDataBits = binaryData.length * EIGHTBIT;
        int fewerThan24bits = lengthDataBits & TWENTYFOURBITGROUP;
        int numberTriplets = lengthDataBits / TWENTYFOURBITGROUP;
        byte [] encodedData = null;
    }



    public static class Pair {
        private Object first;
        private Object second;
        private Object third;

        public Pair() {
        }

        public Pair(Object first, Object second, Object third) {
            this.first = first;
            this.second = second;
            this.third = third;
        }

        public Object getFirst() {
            return first;
        }

        public Object getSecond() {
            return second;
        }

        public Object getThird() {
            return third;
        }

        public void setFirst(Object first) {
            this.first = first;
        }

        public void setSecond(Object second) {
            this.second = second;
        }

        public void setThird(Object third) {
            this.third = third;
        }
    }
}
