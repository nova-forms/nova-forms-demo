package com.ajila.detectors;

import com.ajila.ElementType;

import org.apache.pdfbox.pdmodel.PDDocument;

import java.io.IOException;
import java.util.Map;

/**
 * @author fboesiger
 */
public abstract class Detector {
    public abstract void addElements(PDDocument document, Map<ElementType, Integer> elements) throws IOException;
}
